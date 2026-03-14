use self::state::AppSession;
use crate::core::config::Config;
use crate::core::model::PlaybackState;
use crate::features::player::Player;
use crate::services::storage::Storage;
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
}
