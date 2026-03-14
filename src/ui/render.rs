use std::cmp::min;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph};

use crate::app::App;

use super::state::{FocusPanel, InputMode, UiState};

impl UiState {
    pub fn draw(&mut self, f: &mut ratatui::Frame<'_>, app: &App) {
        let root = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(5),
                Constraint::Length(3),
            ])
            .split(f.area());

        let top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(24), Constraint::Min(20)])
            .split(root[0]);

        self.draw_sidebar(f, top[0]);
        self.draw_library(f, top[1], app);
        self.draw_now_playing(f, root[1], app);
        self.draw_progress(f, root[2], app);
    }

    fn draw_sidebar(&self, f: &mut ratatui::Frame<'_>, area: Rect) {
        let is_focus = matches!(self.focus, FocusPanel::Sidebar);
        let title = if is_focus {
            " Sections [focus] "
        } else {
            " Sections "
        };
        let items = vec![
            ListItem::new("Library"),
            ListItem::new("Search"),
            ListItem::new("Queue"),
            ListItem::new("Now Playing"),
        ];
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(if is_focus {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            });
        let list = List::new(items).block(block);
        f.render_widget(list, area);
    }

    fn draw_library(&mut self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let tracks = self.visible_tracks(app);
        if tracks.is_empty() {
            self.selected = 0;
        } else {
            self.selected = min(self.selected, tracks.len() - 1);
        }

        let is_focus = matches!(self.focus, FocusPanel::Library);
        let mode_title = match self.mode {
            InputMode::Normal => "Normal",
            InputMode::Search => "Search",
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Library [{mode_title}] "))
            .border_style(if is_focus {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            });

        let items: Vec<ListItem<'_>> = tracks
            .iter()
            .map(|track| {
                let marker = if Some(track.id) == app.playback_state().current_track_id {
                    ">"
                } else {
                    " "
                };
                let favorite = if track.favorite { "*" } else { " " };
                ListItem::new(Line::from(format!(
                    "{}{} #{:04} {} - {}",
                    marker,
                    favorite,
                    track.id,
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                    track.title
                )))
            })
            .collect();

        let mut state = ListState::default();
        state.select(if tracks.is_empty() {
            None
        } else {
            Some(self.selected)
        });

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        f.render_stateful_widget(list, area, &mut state);
    }

    fn draw_now_playing(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let current = app.current_track();

        let lines = if let Some(track) = current {
            vec![
                Line::from(format!(
                    "Track: {} - {}",
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                    track.title
                )),
                Line::from(format!(
                    "Album: {} | Repeat: {:?} | Shuffle: {}",
                    track.album.as_deref().unwrap_or("Unknown Album"),
                    app.playback_state().repeat_mode,
                    app.playback_state().shuffle_enabled
                )),
                Line::from(format!("Status: {}", self.status)),
            ]
        } else {
            vec![
                Line::from("Track: (none)"),
                Line::from(format!(
                    "Repeat: {:?} | Shuffle: {}",
                    app.playback_state().repeat_mode,
                    app.playback_state().shuffle_enabled
                )),
                Line::from(format!("Status: {}", self.status)),
            ]
        };

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Now Playing "),
        );
        f.render_widget(paragraph, area);
    }

    fn draw_progress(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let current_duration = app
            .current_track()
            .and_then(|track| track.duration_secs)
            .unwrap_or(0);

        let ratio = if current_duration > 0 {
            (app.playback_state().position_secs as f64 / current_duration as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let label = if self.mode == InputMode::Search {
            format!("/{}", self.search_input)
        } else {
            format!(
                "{}s / {}s",
                app.playback_state().position_secs,
                current_duration
            )
        };

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(" Progress "))
            .gauge_style(Style::default().fg(Color::Magenta))
            .ratio(ratio)
            .label(label);

        f.render_widget(gauge, area);
    }
}
