use super::*;

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
                track_index_by_id: Self::build_track_index(&tracks),
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

    pub fn scan_now(&mut self) -> Result<()> {
        let scan = scan_music_dir(&self.config.music_dir)?;
        self.storage.upsert_tracks(&scan.upserts)?;
        let _removed = self.storage.prune_missing_tracks(&scan.seen_paths)?;
        self.reload_session_state()?;
        Ok(())
    }
}
