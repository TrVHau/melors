use std::collections::{HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::{Duration, SystemTime};

use anyhow::Result;

use crate::core::model::{PlaybackState, RepeatMode, Track};
use crate::services::scanner::scan_music_dir;
use crate::ui;

use super::App;
use super::state::AppSession;

fn shuffle_vec(ids: &mut [i64], current_id: Option<i64>) {
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    let mut state = hasher.finish();

    let n = ids.len();
    for i in (1..n).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let j = (state as usize) % (i + 1);
        ids.swap(i, j);
    }

    // Keep the currently playing track at index 0 so next_track advances correctly.
    if let Some(id) = current_id
        && let Some(pos) = ids.iter().position(|&x| x == id)
    {
        ids.swap(0, pos);
    }
}

const PLAYBACK_PERSIST_DEBOUNCE: Duration = Duration::from_millis(250);

impl App {
    pub fn boot() -> Result<Self> {
        let config = crate::core::config::Config::load_or_create()?;
        let mut storage = crate::services::storage::Storage::open(&config.db_path)?;

        // Boot-time scan keeps DB consistent with file system before UI starts.
        let scan = scan_music_dir(&config.music_dir)?;
        storage.upsert_tracks(&scan.upserts)?;
        let _removed = storage.prune_missing_tracks(&scan.seen_paths)?;

        let tracks = storage.load_tracks()?;
        let queue = storage.load_queue()?;
        let playback_state = storage.load_playback_state()?;
        let player = crate::features::player::Player::new()?;

        let mut app = Self {
            config,
            storage,
            player,
            session: AppSession {
                tracks,
                tracks_version: 1,
                queue,
                queue_version: 1,
                playback_state,
            },
            playback_state_dirty: false,
            last_persisted_playback_state: None,
            last_persisted_at: None,
        };

        app.normalize_queue()?;

        Ok(app)
    }

    pub fn run_tui(&mut self) -> Result<()> {
        ui::run(self)
    }

    pub fn persist_playback_state(&mut self) -> Result<()> {
        self.playback_state_dirty = true;
        self.flush_playback_state(false)
    }

    pub fn persist_playback_state_now(&mut self) -> Result<()> {
        self.playback_state_dirty = true;
        self.flush_playback_state(true)
    }

    pub fn scan_now(&mut self) -> Result<()> {
        let scan = scan_music_dir(&self.config.music_dir)?;
        self.storage.upsert_tracks(&scan.upserts)?;
        let _removed = self.storage.prune_missing_tracks(&scan.seen_paths)?;
        self.reload_session_state()?;
        Ok(())
    }

    pub fn play_track(&mut self, track_id: i64) -> Result<()> {
        self.ensure_track_in_queue(track_id)?;

        if let Some(track) = self
            .session
            .tracks
            .iter()
            .find(|track| track.id == track_id)
        {
            self.player.play_file(&track.path, track.mtime, 0)?;
            self.session.playback_state.current_track_id = Some(track_id);
            self.session.playback_state.position_secs = 0;
            self.storage.increment_play_count(track_id)?;
            self.persist_playback_state()?;
        }
        Ok(())
    }

    pub fn next_track(&mut self) -> Result<()> {
        if self.session.queue.is_empty() {
            return Ok(());
        }
        let current_idx = self
            .session
            .playback_state
            .current_track_id
            .and_then(|id| {
                self.session
                    .queue
                    .iter()
                    .position(|queue_id| *queue_id == id)
            })
            .unwrap_or(usize::MAX);

        let next_idx = if current_idx == usize::MAX {
            0
        } else if current_idx + 1 < self.session.queue.len() {
            current_idx + 1
        } else {
            match self.session.playback_state.repeat_mode {
                RepeatMode::RepeatAll => 0,
                RepeatMode::RepeatOne => current_idx,
                RepeatMode::Off => return Ok(()),
            }
        };

        let id = self.session.queue[next_idx];
        self.play_track(id)
    }

    pub fn prev_track(&mut self) -> Result<()> {
        if self.session.queue.is_empty() {
            return Ok(());
        }

        let current_idx = self
            .session
            .playback_state
            .current_track_id
            .and_then(|id| {
                self.session
                    .queue
                    .iter()
                    .position(|queue_id| *queue_id == id)
            })
            .unwrap_or(0);

        let prev_idx = if current_idx == 0 { 0 } else { current_idx - 1 };

        let id = self.session.queue[prev_idx];
        self.play_track(id)
    }

    pub fn seek(&mut self, delta_secs: i64) -> Result<()> {
        let next = self.player.seek_relative(delta_secs)?;
        self.session.playback_state.position_secs = next;
        self.persist_playback_state()?;
        Ok(())
    }

    pub fn toggle_play_pause(&mut self) -> Result<bool> {
        if self.player.has_active_sink() {
            let paused = self.player.toggle_pause();
            return Ok(paused);
        }

        if let Some(track_id) = self.session.playback_state.current_track_id
            && let Some(track) = self.session.tracks.iter().find(|t| t.id == track_id)
        {
            let start_at = self.session.playback_state.position_secs;
            self.player.play_file(&track.path, track.mtime, start_at)?;
            return Ok(false);
        }

        if let Some(first_track) = self.session.tracks.first() {
            self.play_track(first_track.id)?;
            return Ok(false);
        }

        Ok(true)
    }

    pub fn refresh_playback_position(&mut self) -> Result<()> {
        self.player.poll_analysis_results();

        if self.player.consume_track_finished() {
            let previous_track = self.session.playback_state.current_track_id;
            self.next_track()?;

            if self.session.playback_state.current_track_id == previous_track
                && !self.player.has_active_sink()
            {
                self.session.playback_state.current_track_id = None;
                self.session.playback_state.position_secs = 0;
                self.persist_playback_state()?;
            }
        }

        self.session.playback_state.position_secs = self.player.current_position_secs();
        self.flush_playback_state(false)?;
        Ok(())
    }

    pub fn toggle_favorite(&mut self, track_id: i64) -> Result<()> {
        self.storage.toggle_favorite(track_id)?;
        self.reload_session_state()?;
        Ok(())
    }

    pub fn toggle_repeat(&mut self) -> Result<RepeatMode> {
        self.session.playback_state.repeat_mode = self.session.playback_state.repeat_mode.cycle();
        self.persist_playback_state()?;
        Ok(self.session.playback_state.repeat_mode)
    }

    pub fn adjust_volume(&mut self, delta: f32) -> u8 {
        self.player.adjust_volume(delta)
    }

    pub fn volume_percent(&self) -> u8 {
        self.player.volume_percent()
    }

    pub fn visualizer_levels(&self, bars: usize) -> Vec<(f32, f32)> {
        self.player.visualizer_levels(bars)
    }

    pub fn toggle_shuffle(&mut self) -> Result<bool> {
        self.session.playback_state.shuffle_enabled = !self.session.playback_state.shuffle_enabled;
        self.rebuild_queue()?;
        self.persist_playback_state()?;
        Ok(self.session.playback_state.shuffle_enabled)
    }

    pub fn add_to_queue(&mut self, track_id: i64) -> Result<bool> {
        if self.track_by_id(track_id).is_none() {
            return Ok(false);
        }

        self.session.queue.push(track_id);
        self.persist_queue()?;
        Ok(true)
    }

    pub fn remove_queue_index(&mut self, index: usize) -> Result<Option<i64>> {
        if index >= self.session.queue.len() {
            return Ok(None);
        }

        let removed = self.session.queue.remove(index);
        self.persist_queue()?;
        Ok(Some(removed))
    }

    pub fn play_queue_index(&mut self, index: usize) -> Result<Option<i64>> {
        if let Some(track_id) = self.session.queue.get(index).copied() {
            self.play_track(track_id)?;
            return Ok(Some(track_id));
        }

        Ok(None)
    }

    pub fn current_track(&self) -> Option<&Track> {
        self.session
            .playback_state
            .current_track_id
            .and_then(|id| self.session.tracks.iter().find(|track| track.id == id))
    }

    pub fn queue_len(&self) -> usize {
        self.session.queue.len()
    }

    pub fn queue_ids(&self) -> &[i64] {
        &self.session.queue
    }

    pub fn tracks_version(&self) -> u64 {
        self.session.tracks_version
    }

    pub fn queue_version(&self) -> u64 {
        self.session.queue_version
    }

    pub fn tracks(&self) -> &[Track] {
        &self.session.tracks
    }

    pub fn playback_state(&self) -> &PlaybackState {
        &self.session.playback_state
    }

    fn rebuild_queue(&mut self) -> Result<()> {
        let mut ids: Vec<i64> = self.session.tracks.iter().map(|t| t.id).collect();
        if self.session.playback_state.shuffle_enabled {
            shuffle_vec(&mut ids, self.session.playback_state.current_track_id);
        }
        self.session.queue = ids;
        self.persist_queue()?;
        Ok(())
    }

    fn reload_session_state(&mut self) -> Result<()> {
        self.session.tracks = self.storage.load_tracks()?;
        self.session.tracks_version = self.session.tracks_version.saturating_add(1);
        self.session.queue = self.storage.load_queue()?;
        self.session.queue_version = self.session.queue_version.saturating_add(1);
        self.normalize_queue()?;

        let current_track_missing = self
            .session
            .playback_state
            .current_track_id
            .is_some_and(|track_id| !self.session.tracks.iter().any(|track| track.id == track_id));

        if current_track_missing {
            self.player.stop();
            self.session.playback_state.current_track_id = None;
            self.session.playback_state.position_secs = 0;
            self.persist_playback_state()?;
        }

        Ok(())
    }

    fn normalize_queue(&mut self) -> Result<()> {
        let valid_ids: HashSet<i64> = self.session.tracks.iter().map(|track| track.id).collect();
        let original_len = self.session.queue.len();
        self.session.queue.retain(|id| valid_ids.contains(id));

        if self.session.queue.is_empty() && !self.session.tracks.is_empty() {
            self.rebuild_queue()?;
            return Ok(());
        }

        if self.session.queue.len() != original_len {
            self.persist_queue()?;
        }

        Ok(())
    }

    fn ensure_track_in_queue(&mut self, track_id: i64) -> Result<()> {
        if self.session.queue.contains(&track_id) {
            return Ok(());
        }

        self.session.queue.push(track_id);
        self.persist_queue()?;
        Ok(())
    }

    fn persist_queue(&mut self) -> Result<()> {
        self.storage.replace_queue(&self.session.queue)?;
        self.session.queue_version = self.session.queue_version.saturating_add(1);
        Ok(())
    }

    pub fn track_by_id(&self, track_id: i64) -> Option<&Track> {
        self.session
            .tracks
            .iter()
            .find(|track| track.id == track_id)
    }

    pub fn rename_track(&mut self, track_id: i64, new_title: &str) -> Result<()> {
        let (old_path, new_path) = {
            let track = match self.track_by_id(track_id) {
                Some(t) => t,
                None => return Ok(()),
            };
            let old_path = track.path.clone();
            let ext = old_path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let new_filename = if ext.is_empty() {
                new_title.to_string()
            } else {
                format!("{}.{}", new_title, ext)
            };
            let new_path = old_path
                .parent()
                .unwrap_or(Path::new("."))
                .join(&new_filename);
            (old_path, new_path)
        };
        std::fs::rename(&old_path, &new_path)?;
        self.storage.rename_track(track_id, new_title, &new_path.to_string_lossy())?;
        self.reload_session_state()
    }

    pub fn rename_artist(&mut self, track_id: i64, new_artist: &str) -> Result<()> {
        if self.track_by_id(track_id).is_none() {
            return Ok(());
        }
        self.storage.rename_artist(track_id, new_artist)?;
        self.reload_session_state()
    }

    fn flush_playback_state(&mut self, force: bool) -> Result<()> {
        if !self.playback_state_dirty {
            return Ok(());
        }

        let unchanged = self
            .last_persisted_playback_state
            .as_ref()
            .is_some_and(|last| *last == self.session.playback_state);
        if unchanged {
            self.playback_state_dirty = false;
            return Ok(());
        }

        if !force
            && self
                .last_persisted_at
                .is_some_and(|at| at.elapsed() < PLAYBACK_PERSIST_DEBOUNCE)
        {
            return Ok(());
        }

        self.storage
            .save_playback_state(&self.session.playback_state)?;
        self.last_persisted_playback_state = Some(self.session.playback_state.clone());
        self.last_persisted_at = Some(std::time::Instant::now());
        self.playback_state_dirty = false;
        Ok(())
    }
}
