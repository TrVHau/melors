use std::cmp::min;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Local;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph};

use crate::app::App;

use super::state::{FocusPanel, InputMode, UiState, VisualizerMode};

impl UiState {
    pub fn draw(&mut self, f: &mut ratatui::Frame<'_>, app: &App) {
        let root = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(8),
                Constraint::Length(3),
            ])
            .split(f.area());

        let top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(18),
                Constraint::Min(32),
                Constraint::Length(36),
            ])
            .split(root[0]);

        self.draw_sidebar(f, top[0]);
        self.draw_library(f, top[1], app);
        self.draw_queue(f, top[2], app);

        let bottom = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(root[1]);

        self.draw_now_playing(f, bottom[0], app);
        self.draw_visualizer_panel(f, bottom[1], app);
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
            ListItem::new(if matches!(self.focus, FocusPanel::Library) {
                "> Library"
            } else {
                "  Library"
            }),
            ListItem::new(if self.mode == InputMode::Search {
                "> Search"
            } else {
                "  Search"
            }),
            ListItem::new(if matches!(self.focus, FocusPanel::Queue) {
                "> Queue"
            } else {
                "  Queue"
            }),
            ListItem::new("  Now Playing"),
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
            self.library_selected = 0;
        } else {
            self.library_selected = min(self.library_selected, tracks.len() - 1);
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
            Some(self.library_selected)
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

    fn draw_queue(&mut self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let queue_tracks = app.queue_tracks();
        if queue_tracks.is_empty() {
            self.queue_selected = 0;
        } else {
            self.queue_selected = min(self.queue_selected, queue_tracks.len() - 1);
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Queue ")
            .border_style(if matches!(self.focus, FocusPanel::Queue) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        let items: Vec<ListItem<'_>> = if queue_tracks.is_empty() {
            vec![ListItem::new(Line::from("(queue empty)"))]
        } else {
            queue_tracks
                .iter()
                .enumerate()
                .map(|(idx, track)| {
                    let marker = if Some(track.id) == app.playback_state().current_track_id {
                        ">"
                    } else {
                        " "
                    };
                    ListItem::new(Line::from(format!(
                        "{} {:02}. {} - {}",
                        marker,
                        idx + 1,
                        track.artist.as_deref().unwrap_or("Unknown Artist"),
                        track.title
                    )))
                })
                .collect()
        };

        let mut state = ListState::default();
        state.select(if queue_tracks.is_empty() {
            None
        } else {
            Some(self.queue_selected)
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
                    "Album: {} | Repeat: {} | Shuffle: {}",
                    track.album.as_deref().unwrap_or("Unknown Album"),
                    app.playback_state().repeat_mode,
                    if app.playback_state().shuffle_enabled {
                        "On"
                    } else {
                        "Off"
                    }
                )),
                Line::from(format!("Volume: {}%", app.volume_percent())),
                Line::from(format!("Status: {}", self.status)),
            ]
        } else {
            vec![
                Line::from("Track: (none)"),
                Line::from(format!(
                    "Repeat: {} | Shuffle: {}",
                    app.playback_state().repeat_mode,
                    if app.playback_state().shuffle_enabled {
                        "On"
                    } else {
                        "Off"
                    }
                )),
                Line::from(format!("Volume: {}%", app.volume_percent())),
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

    fn draw_visualizer_panel(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        match self.visualizer_mode {
            VisualizerMode::Cava => self.draw_cava_visualizer(f, area, app),
            VisualizerMode::Clock => self.draw_clock_visualizer(f, area),
            VisualizerMode::CMatrix => self.draw_cmatrix_visualizer(f, area, app),
        }
    }

    fn draw_cava_visualizer(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let inner = area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 1,
        });
        let width = inner.width.max(1) as usize;
        let height = inner.height.max(1) as usize;
        let bars = (width / 2).max(1);
        let tick = self.visualizer_tick();
        let position = app.playback_state().position_secs as f64;

        let heights: Vec<usize> = (0..bars)
            .map(|idx| {
                let wave_a = ((tick / 180.0) + idx as f64 * 0.55 + position * 0.18).sin();
                let wave_b = ((tick / 120.0) + idx as f64 * 0.21 + position * 0.09).cos();
                let energy = ((wave_a + wave_b + 2.0) / 4.0).clamp(0.0, 1.0);
                ((energy * height as f64).round() as usize).clamp(1, height)
            })
            .collect();

        let mut lines = Vec::with_capacity(height + 1);
        for row in (0..height).rev() {
            let spans: Vec<Span<'_>> = heights
                .iter()
                .map(|bar_height| {
                    if *bar_height > row {
                        Span::styled("##", Style::default().fg(Color::Cyan))
                    } else {
                        Span::raw("  ")
                    }
                })
                .collect();
            lines.push(Line::from(spans));
        }
        lines.push(Line::from(vec![Span::styled(
            "Alt+1 Cava  Alt+2 Clock  Alt+3 CMatrix",
            Style::default().fg(Color::DarkGray),
        )]));

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Visualizer [Cava] "),
        );
        f.render_widget(paragraph, area);
    }

    fn draw_clock_visualizer(&self, f: &mut ratatui::Frame<'_>, area: Rect) {
        let now = Local::now();
        let lines = vec![
            self.centered_line(area.width, "Clock Mode", Color::Yellow),
            Line::default(),
            self.centered_line(
                area.width,
                &now.format("%H:%M:%S").to_string(),
                Color::White,
            ),
            self.centered_line(area.width, &now.format("%Y-%m-%d").to_string(), Color::Cyan),
            Line::default(),
            self.centered_line(area.width, "Alt+1 / Alt+2 / Alt+3", Color::DarkGray),
        ];

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Visualizer [Clock] "),
        );
        f.render_widget(paragraph, area);
    }

    fn draw_cmatrix_visualizer(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let inner = area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 1,
        });
        let width = inner.width.max(1) as usize;
        let height = inner.height.max(1) as usize;
        let tick = self.visualizer_tick() as usize;
        let position = app.playback_state().position_secs.max(0) as usize;
        let charset = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let trail = 6usize;

        let mut lines = Vec::with_capacity(height);
        for row in 0..height {
            let mut spans = Vec::with_capacity(width);
            for col in 0..width {
                let head = (tick / 2 + col * 5 + position) % (height + trail);
                if row <= head && head - row < trail {
                    let index = (tick + row * 11 + col * 7 + position) % charset.len();
                    let ch = charset[index] as char;
                    let color = if row == head {
                        Color::White
                    } else if head - row <= 2 {
                        Color::Green
                    } else {
                        Color::DarkGray
                    };
                    spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                } else {
                    spans.push(Span::raw(" "));
                }
            }
            lines.push(Line::from(spans));
        }

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Visualizer [CMatrix] "),
        );
        f.render_widget(paragraph, area);
    }

    fn visualizer_tick(&self) -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as f64
    }

    fn centered_line(&self, width: u16, text: &str, color: Color) -> Line<'static> {
        let available = width.saturating_sub(2) as usize;
        let left_pad = available.saturating_sub(text.len()) / 2;
        let mut content = String::new();
        content.push_str(&" ".repeat(left_pad));
        content.push_str(text);
        Line::from(vec![Span::styled(content, Style::default().fg(color))])
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
