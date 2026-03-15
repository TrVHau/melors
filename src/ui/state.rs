use std::cmp::min;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

use fuzzy_matcher::skim::SkimMatcherV2;

use crate::app::App;
use crate::features::search::search_tracks;

mod cache;
mod mode;
mod model;

pub use model::{FocusPanel, InputMode, RenameKind, UiState, UiTheme, VisualizerMode};
