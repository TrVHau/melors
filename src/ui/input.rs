use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

use super::state::{FocusPanel, InputMode, RenameKind, UiState, VisualizerMode};

mod dispatch;
mod modes;
mod normal;
mod selection;
