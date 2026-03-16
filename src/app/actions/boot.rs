use super::*;
use std::sync::mpsc::TryRecvError;

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
            scan_rx: None,
            scan_in_progress: false,
        };

        app.normalize_queue()?;

        Ok(app)
    }

    pub fn run_tui(&mut self) -> Result<()> {
        ui::run(self)
    }

    pub fn begin_scan(&mut self) -> bool {
        if self.scan_in_progress {
            return false;
        }

        let music_dir = self.config.music_dir.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = scan_music_dir(&music_dir).map_err(|e| e.to_string());
            let _ = tx.send(result);
        });

        self.scan_rx = Some(rx);
        self.scan_in_progress = true;
        true
    }

    pub fn poll_scan_status(&mut self) -> Option<String> {
        let recv = {
            let rx = self.scan_rx.as_ref()?;
            rx.try_recv()
        };

        match recv {
            Ok(Ok(scan)) => {
                self.scan_in_progress = false;
                self.scan_rx = None;
                match self.apply_scan_result(scan) {
                    Ok((upserts, removed)) => Some(format!(
                        "Rescan complete ({} scanned, {} removed)",
                        upserts, removed
                    )),
                    Err(err) => Some(format!("Rescan failed: {err}")),
                }
            }
            Ok(Err(err)) => {
                self.scan_in_progress = false;
                self.scan_rx = None;
                Some(format!("Rescan failed: {err}"))
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                self.scan_in_progress = false;
                self.scan_rx = None;
                Some(String::from("Rescan failed: worker disconnected"))
            }
        }
    }

    fn apply_scan_result(
        &mut self,
        scan: crate::services::scanner::ScanResult,
    ) -> Result<(usize, usize)> {
        let upserts = scan.upserts.len();
        self.storage.upsert_tracks(&scan.upserts)?;
        let removed = self.storage.prune_missing_tracks(&scan.seen_paths)?;
        self.reload_session_state()?;
        Ok((upserts, removed))
    }
}
