use self::state::AppSession;
use crate::core::config::Config;
use crate::core::model::PlaybackState;
use crate::features::player::Player;
use crate::services::scanner::{ScanResult, WatcherEventBatch};
use crate::services::storage::Storage;
use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::time::Instant;

mod actions;
pub mod state;

pub struct App {
    pub(crate) config: Config,
    pub(crate) storage: Storage,
    pub(crate) player: Player,
    pub(crate) session: AppSession,
    pub(crate) playback_state_dirty: bool,
    pub(crate) last_persisted_playback_state: Option<PlaybackState>,
    pub(crate) last_persisted_at: Option<Instant>,
    pub(crate) scan_rx: Option<Receiver<std::result::Result<ScanResult, String>>>,
    pub(crate) watcher_rx: Option<Receiver<std::result::Result<WatcherEventBatch, String>>>,
    pub(crate) scan_in_progress: bool,
    pub(crate) scan_pending_rescan: bool,
    pub(crate) scan_pending_dirty_paths: HashSet<String>,
    pub(crate) scan_generation: u64,
    pub(crate) scan_last_completed_generation: u64,
}
