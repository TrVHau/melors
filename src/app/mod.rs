use self::state::AppSession;
use crate::core::config::Config;
use crate::features::player::Player;
use crate::services::storage::Storage;

mod actions;
pub mod state;

pub struct App {
    pub(crate) config: Config,
    pub(crate) storage: Storage,
    pub(crate) player: Player,
    pub(crate) session: AppSession,
}
