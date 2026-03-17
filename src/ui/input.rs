use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

use super::state::{FocusPanel, InputMode, UiState, VisualizerMode};

mod dispatch;
mod modes;
mod normal;
mod selection;
