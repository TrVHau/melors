use anyhow::Result;

use crate::core::model::{PlaybackState, RepeatMode, Track};
use crate::services::scanner::scan_music_dir;
use crate::ui;

use super::App;
use super::state::AppSession;

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

        Ok(Self {
            config,
            storage,
            player,
            session: AppSession {
                tracks,
                queue,
                playback_state,
            },
        })
    }

    pub fn run_tui(&mut self) -> Result<()> {
        ui::run(self)
    }

    pub fn persist_playback_state(&self) -> Result<()> {
        self.storage
            .save_playback_state(&self.session.playback_state)
    }

    pub fn scan_now(&mut self) -> Result<()> {
        let scan = scan_music_dir(&self.config.music_dir)?;
        self.storage.upsert_tracks(&scan.upserts)?;
        let _removed = self.storage.prune_missing_tracks(&scan.seen_paths)?;
        self.reload_session_state()?;
        Ok(())
    }

    pub fn play_track(&mut self, track_id: i64) -> Result<()> {
        if let Some(track) = self
            .session
            .tracks
            .iter()
            .find(|track| track.id == track_id)
        {
            self.player.play_file(&track.path, 0)?;
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
            self.player.play_file(&track.path, start_at)?;
            return Ok(false);
        }

        if let Some(first_track) = self.session.tracks.first() {
            self.play_track(first_track.id)?;
            return Ok(false);
        }

        Ok(true)
    }

    pub fn refresh_playback_position(&mut self) -> Result<()> {
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
        Ok(())
    }

    pub fn toggle_favorite(&mut self, track_id: i64) -> Result<()> {
        self.storage.toggle_favorite(track_id)?;
        self.reload_session_state()?;
        Ok(())
    }

    pub fn current_track(&self) -> Option<&Track> {
        self.session
            .playback_state
            .current_track_id
            .and_then(|id| self.session.tracks.iter().find(|track| track.id == id))
    }

    pub fn tracks(&self) -> &[Track] {
        &self.session.tracks
    }

    pub fn playback_state(&self) -> &PlaybackState {
        &self.session.playback_state
    }

    fn reload_session_state(&mut self) -> Result<()> {
        self.session.tracks = self.storage.load_tracks()?;
        self.session.queue = self.storage.load_queue()?;

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
}
