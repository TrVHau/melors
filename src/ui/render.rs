use std::cmp::min;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Local;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Tabs};

use crate::app::App;

use super::state::{FocusPanel, InputMode, RenameKind, UiState, UiTheme, VisualizerMode};

mod chrome;
mod lists;
mod playback;
mod theme;
mod visualizer;
