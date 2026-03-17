use super::*;
use std::collections::HashSet;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;

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
        let watcher_rx = crate::services::scanner::spawn_watcher_runtime(
            config.music_dir.clone(),
            Duration::from_millis(250),
            Duration::from_millis(150),
        )?;

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
            watcher_rx: Some(watcher_rx),
            scan_in_progress: false,
            scan_pending_rescan: false,
            scan_pending_dirty_paths: HashSet::new(),
            scan_generation: 0,
            scan_last_completed_generation: 0,
        };

        app.normalize_queue()?;

        Ok(app)
    }

    pub fn run_tui(&mut self) -> Result<()> {
        ui::run(self)
    }

    pub fn begin_scan(&mut self) -> bool {
        self.request_scan(HashSet::new())
    }

    fn request_scan(&mut self, dirty_paths: HashSet<String>) -> bool {
        self.scan_pending_dirty_paths.extend(dirty_paths);
        if self.scan_in_progress {
            self.scan_pending_rescan = true;
            return false;
        }

        self.scan_in_progress = true;
        self.scan_generation = self.scan_generation.saturating_add(1);
        self.scan_pending_rescan = false;
        self.scan_pending_dirty_paths.clear();
        self.spawn_scan_worker();
        true
    }

    fn spawn_scan_worker(&mut self) {
        let music_dir = self.config.music_dir.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = scan_music_dir(&music_dir).map_err(|e| e.to_string());
            let _ = tx.send(result);
        });
        self.scan_rx = Some(rx);
    }

    pub fn poll_scan_status(&mut self) -> Option<String> {
        let watcher_status = self.poll_watcher_batches();
        let recv = {
            let rx = self.scan_rx.as_ref()?;
            rx.try_recv()
        };

        match recv {
            Ok(Ok(scan)) => {
                self.scan_in_progress = false;
                self.scan_rx = None;
                match self.apply_scan_result(scan) {
                    Ok((upserts, removed, warnings)) => {
                        self.scan_last_completed_generation = self.scan_generation;
                        let mut message = if warnings > 0 {
                            format!(
                                "Rescan complete ({} scanned, {} removed, {} warnings)",
                                upserts, removed, warnings
                            )
                        } else {
                            format!("Rescan complete ({} scanned, {} removed)", upserts, removed)
                        };
                        if self.scan_pending_rescan || !self.scan_pending_dirty_paths.is_empty() {
                            let pending = self.scan_pending_dirty_paths.len();
                            let next_dirty = std::mem::take(&mut self.scan_pending_dirty_paths);
                            self.scan_pending_rescan = false;
                            let _started = self.request_scan(next_dirty);
                            message.push_str(&format!(
                                "; follow-up scan queued ({} pending paths)",
                                pending
                            ));
                        }
                        Some(message)
                    }
                    Err(err) => Some(format!("Rescan failed: {err}")),
                }
            }
            Ok(Err(err)) => {
                self.scan_in_progress = false;
                self.scan_rx = None;
                let mut message = format!("Rescan failed: {err}");
                if self.scan_pending_rescan || !self.scan_pending_dirty_paths.is_empty() {
                    let pending = self.scan_pending_dirty_paths.len();
                    let next_dirty = std::mem::take(&mut self.scan_pending_dirty_paths);
                    self.scan_pending_rescan = false;
                    let _started = self.request_scan(next_dirty);
                    message.push_str(&format!("; retry scan queued ({} pending paths)", pending));
                }
                Some(message)
            }
            Err(TryRecvError::Empty) => watcher_status,
            Err(TryRecvError::Disconnected) => {
                self.scan_in_progress = false;
                self.scan_rx = None;
                Some(String::from("Rescan failed: worker disconnected"))
            }
        }
    }

    fn poll_watcher_batches(&mut self) -> Option<String> {
        let mut latest = None;
        let rx = self.watcher_rx.take()?;
        let mut disconnected = false;
        loop {
            match rx.try_recv() {
                Ok(Ok(batch)) => {
                    self.scan_generation =
                        self.scan_generation.max(batch.generation.saturating_sub(1));
                    let started = self.request_scan(batch.dirty_paths.clone());
                    latest = Some(if started {
                        format!(
                            "Watcher refresh scheduled (gen {}, {} changes, {}ms window)",
                            batch.generation,
                            batch.dirty_paths.len(),
                            batch.window_elapsed_ms
                        )
                    } else {
                        format!(
                            "Watcher changes queued (gen {}, {} changes)",
                            batch.generation,
                            batch.dirty_paths.len()
                        )
                    });
                }
                Ok(Err(err)) => latest = Some(format!("Watcher error: {err}")),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    disconnected = true;
                    break;
                }
            }
        }
        if !disconnected {
            self.watcher_rx = Some(rx);
        }
        latest
    }

    fn apply_scan_result(
        &mut self,
        scan: crate::services::scanner::ScanResult,
    ) -> Result<(usize, usize, usize)> {
        let upserts = scan.upserts.len();
        let warnings = scan.warnings.failed_files;
        self.storage.upsert_tracks(&scan.upserts)?;
        let removed = self.storage.prune_missing_tracks(&scan.seen_paths)?;
        self.reload_session_state()?;
        Ok((upserts, removed, warnings))
    }
}
