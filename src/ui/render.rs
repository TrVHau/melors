use std::cmp::min;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Local;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph};

use crate::app::App;

use super::state::{FocusPanel, InputMode, UiState, VisualizerMode};

impl UiState {
    pub fn draw(&mut self, f: &mut ratatui::Frame<'_>, app: &App) {
        f.render_widget(Clear, f.area());

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
        f.render_widget(Clear, area);

        let rows_len = self.library_rows(app).len();
        if rows_len == 0 {
            self.library_selected = 0;
        } else {
            self.library_selected = min(self.library_selected, rows_len - 1);
        }

        let is_focus = matches!(self.focus, FocusPanel::Library);
        let mode_title = match self.mode {
            InputMode::Normal => "Normal",
            InputMode::Search => "Search",
            InputMode::Rename => "Rename",
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Library [{mode_title}] "))
            .border_style(if is_focus {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            });

        let content_width = area.width.saturating_sub(4) as usize;
        let items: Vec<ListItem<'_>> = self
            .library_rows(app)
            .iter()
            .map(|row| {
                let text = Self::fixed_width_text(row, content_width);
                ListItem::new(Line::from(text))
            })
            .collect();

        let mut state = ListState::default();
        state.select(if rows_len == 0 {
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
        f.render_widget(Clear, area);

        let queue_len = self.queue_rows(app).len();
        if queue_len == 0 {
            self.queue_selected = 0;
        } else {
            self.queue_selected = min(self.queue_selected, queue_len - 1);
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Queue ")
            .border_style(if matches!(self.focus, FocusPanel::Queue) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        let content_width = area.width.saturating_sub(4) as usize;
        let items: Vec<ListItem<'_>> = if queue_len == 0 {
            vec![ListItem::new(Line::from(Self::fixed_width_text(
                "(queue empty)",
                content_width,
            )))]
        } else {
            self.queue_rows(app)
                .iter()
                .map(|row| {
                    let text = Self::fixed_width_text(row, content_width);
                    ListItem::new(Line::from(text))
                })
                .collect()
        };

        let mut state = ListState::default();
        state.select(if queue_len == 0 {
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

    fn draw_visualizer_panel(&mut self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        match self.visualizer_mode {
            VisualizerMode::Cava => self.draw_cava_visualizer(f, area, app),
            VisualizerMode::Clock => self.draw_clock_visualizer(f, area),
            VisualizerMode::CMatrix => self.draw_cmatrix_visualizer(f, area, app),
        }
    }

    fn draw_cava_visualizer(&mut self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let inner = self.visualizer_inner(area);
        let width = inner.width.max(1) as usize;
        let height = inner.height.max(1) as usize;
        let bars = ((width + 1) / 3).clamp(1, 40);
        let tick_ms = self.visualizer_tick() as u128;
        if self.cava_cached_levels.len() != bars
            || tick_ms.saturating_sub(self.visualizer_last_update_ms) >= 33
        {
            let fresh = app.visualizer_levels(bars);
            if self.cava_cached_levels.len() == fresh.len() {
                for (idx, new_value) in fresh.iter().enumerate() {
                    let old = self.cava_cached_levels[idx];
                    let level = old.0 * 0.65 + new_value.0 * 0.35;
                    let peak = new_value.1.max(old.1 * 0.92);
                    self.cava_cached_levels[idx] = (level, peak);
                }
            } else {
                self.cava_cached_levels = fresh;
            }
            self.visualizer_last_update_ms = tick_ms;
        }

        let active_height = height.saturating_sub(1).max(1);

        let mut lines = Vec::with_capacity(height);
        for row in (0..active_height).rev() {
            let mut row_text = String::with_capacity(bars * 3);
            for (level, peak) in &self.cava_cached_levels {
                let bar_height =
                    ((level * active_height as f32).round() as usize).clamp(0, active_height);
                let peak_height =
                    ((peak * active_height as f32).round() as usize).clamp(0, active_height);

                if peak_height == row + 1 && peak_height > 0 {
                    row_text.push_str("▓▓ ");
                } else if bar_height > row {
                    row_text.push_str("██ ");
                } else {
                    row_text.push_str("   ");
                }
            }

            lines.push(Line::from(vec![Span::styled(
                row_text,
                Style::default().fg(Color::Rgb(236, 236, 245)),
            )]));
        }

        let baseline = "-  ".repeat(bars);
        lines.push(Line::from(vec![Span::styled(
            baseline,
            Style::default().fg(Color::Rgb(150, 150, 170)),
        )]));

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Visualizer [Cava] ")
                .border_style(Style::default().fg(Color::Rgb(178, 178, 210))),
        );
        f.render_widget(paragraph, area);
    }

    fn draw_clock_visualizer(&self, f: &mut ratatui::Frame<'_>, area: Rect) {
        let now = Local::now();
        let inner = self.visualizer_inner(area);
        let available_rows = inner.height.max(1) as usize;

        let mut clock_rows = self.big_clock_lines(&now.format("%H:%M:%S").to_string());
        if clock_rows.len() > available_rows {
            clock_rows = self.sample_rows(&clock_rows, available_rows);
        }

        let mut lines = Vec::new();
        let can_show_date = available_rows >= clock_rows.len() + 2;
        if can_show_date {
            lines.push(self.centered_line(
                area.width,
                &now.format("%A %Y-%m-%d").to_string(),
                Color::Rgb(245, 185, 175),
            ));
            lines.push(Line::default());
        }

        for line in clock_rows {
            lines.push(self.centered_line(area.width, &line, Color::Rgb(244, 184, 180)));
        }

        while lines.len() < available_rows {
            lines.push(Line::default());
        }

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Visualizer [Clock] ")
                .border_style(Style::default().fg(Color::Rgb(204, 156, 164))),
        );
        f.render_widget(paragraph, area);
    }

    fn draw_cmatrix_visualizer(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let inner = self.visualizer_inner(area);
        let width = inner.width.max(1) as usize;
        let height = inner.height.max(1) as usize;
        let tick = self.visualizer_tick() as usize;
        let position = app.playback_state().position_secs.max(0) as usize;
        let charset = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let seed = self.track_seed(app) as usize;

        let mut lines = Vec::with_capacity(height);
        for row in 0..height {
            let mut spans = Vec::with_capacity(width);
            for col in 0..width {
                let speed = 1 + ((seed + col * 13) % 4);
                let trail = 4 + ((seed / 7 + col * 3) % 8);
                let offset = (seed / 11 + col * 17 + position * 3) % (height + trail + 8);
                let head = (tick / speed + offset) % (height + trail + 8);

                if row <= head && head - row < trail {
                    let glyph_index =
                        (seed + tick + row * 19 + col * 23 + position) % charset.len();
                    let ch = charset[glyph_index] as char;
                    let distance = head - row;
                    let color = if distance == 0 {
                        Color::White
                    } else if distance <= 2 {
                        Color::Rgb(120, 255, 120)
                    } else {
                        Color::Rgb(0, 120, 0)
                    };
                    spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                } else if (seed + tick + row * 5 + col * 3).is_multiple_of(37) {
                    spans.push(Span::styled("·", Style::default().fg(Color::Rgb(0, 60, 0))));
                } else {
                    spans.push(Span::raw(" "));
                }
            }
            lines.push(Line::from(spans));
        }

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Visualizer [CMatrix] ")
                .border_style(Style::default().fg(Color::Rgb(0, 180, 0))),
        );
        f.render_widget(paragraph, area);
    }

    fn visualizer_tick(&self) -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as f64
    }

    fn visualizer_inner(&self, area: Rect) -> Rect {
        area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 1,
        })
    }

    fn track_seed(&self, app: &App) -> u64 {
        let mut hasher = DefaultHasher::new();
        if let Some(track) = app.current_track() {
            track.id.hash(&mut hasher);
            track.path.hash(&mut hasher);
            track.title.hash(&mut hasher);
            track.artist.hash(&mut hasher);
            track.album.hash(&mut hasher);
            track.duration_secs.hash(&mut hasher);
        } else {
            app.playback_state().position_secs.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn big_clock_lines(&self, text: &str) -> Vec<String> {
        const HEIGHT: usize = 5;
        let mut lines = vec![String::new(); HEIGHT];

        for ch in text.chars() {
            let glyph = match ch {
                '0' => ["███", "█ █", "█ █", "█ █", "███"],
                '1' => [" ██", "███", " ██", " ██", "███"],
                '2' => ["███", "  █", "███", "█  ", "███"],
                '3' => ["███", "  █", "███", "  █", "███"],
                '4' => ["█ █", "█ █", "███", "  █", "  █"],
                '5' => ["███", "█  ", "███", "  █", "███"],
                '6' => ["███", "█  ", "███", "█ █", "███"],
                '7' => ["███", "  █", "  █", "  █", "  █"],
                '8' => ["███", "█ █", "███", "█ █", "███"],
                '9' => ["███", "█ █", "███", "  █", "███"],
                ':' => ["   ", " █ ", "   ", " █ ", "   "],
                _ => ["   ", "   ", "   ", "   ", "   "],
            };

            for (row, part) in glyph.iter().enumerate() {
                lines[row].push_str(part);
                lines[row].push(' ');
            }
        }

        lines
    }

    fn sample_rows(&self, rows: &[String], target: usize) -> Vec<String> {
        if target == 0 {
            return Vec::new();
        }
        if rows.len() <= target {
            return rows.to_vec();
        }

        (0..target)
            .map(|idx| {
                let src = idx * rows.len() / target;
                rows[src].clone()
            })
            .collect()
    }

    fn centered_line(&self, width: u16, text: &str, color: Color) -> Line<'static> {
        let available = width.saturating_sub(2) as usize;
        let text_width = text.chars().count();
        let left_pad = available.saturating_sub(text_width) / 2;
        let mut content = String::new();
        content.push_str(&" ".repeat(left_pad));
        content.push_str(text);
        Line::from(vec![Span::styled(content, Style::default().fg(color))])
    }

    fn draw_progress(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        f.render_widget(Clear, area);

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
        } else if self.mode == InputMode::Rename {
            let track_id_label = self
                .rename_track_id
                .map(|id| format!("#{} ", id))
                .unwrap_or_default();
            format!("Rename {}→ {}_", track_id_label, self.rename_input)
        } else {
            format!(
                "{}s / {}s",
                app.playback_state().position_secs,
                current_duration
            )
        };

        let label_width = area.width.saturating_sub(2) as usize;
        let label = Self::fixed_width_text(&label, label_width);

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(" Progress "))
            .gauge_style(Style::default().fg(Color::Magenta))
            .ratio(ratio)
            .label(label);

        f.render_widget(gauge, area);
    }

    fn fixed_width_text(text: &str, width: usize) -> String {
        if width == 0 {
            return String::new();
        }

        let mut out: String = text.chars().take(width).collect();
        let used = out.chars().count();
        if used < width {
            out.push_str(&" ".repeat(width - used));
        }
        out
    }
}
