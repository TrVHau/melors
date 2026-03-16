use std::cmp::min;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::app::App;

mod cache;
mod mode;
mod model;
mod playlists;

pub use model::{
    FocusPanel, InputMode, PlaylistModalMode, RenameKind, UiState, UiTheme, VisualizerMode,
};
