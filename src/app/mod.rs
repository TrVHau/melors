use self::state::AppSession;
use crate::core::config::Config;
use crate::core::model::PlaybackState;
use crate::features::player::Player;
use crate::services::scanner::ScanResult;
use crate::services::storage::Storage;
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
    pub(crate) scan_in_progress: bool,
}
