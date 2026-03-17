use super::*;

impl UiState {
    pub(super) fn draw_now_playing(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let current = app.current_track();

        let lines = if let Some(track) = current {
            vec![
                Line::from(format!(
                    "{} - {}",
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                    track.title
                )),
                Line::from(format!(
                    "Repeat={} Shuffle={}",
                    app.playback_state().repeat_mode,
                    if app.playback_state().shuffle_enabled {
                        "On"
                    } else {
                        "Off"
                    }
                )),
                Line::from(format!(
                    "Vol {}%",
                    app.volume_percent()
                )),
            ]
        } else {
            vec![
                Line::from("Track: (none)"),
                Line::from(format!(
                    "Repeat={} Shuffle={}",
                    app.playback_state().repeat_mode,
                    if app.playback_state().shuffle_enabled {
                        "On"
                    } else {
                        "Off"
                    }
                )),
                Line::from(self.next_up_line(app)),
                Line::from(format!("Vol {}%", app.volume_percent())),
            ]
        };

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Now Playing ")
                    .style(Style::default().bg(self.theme_panel_bg_color()))
                    .border_style(if app.is_actively_playing() {
                        Style::default().fg(self.theme_library_color())
                    } else if app.current_track().is_some() {
                        Style::default().fg(self.theme_queue_color())
                    } else {
                        Style::default().fg(self.theme_dim_color())
                    }),
            )
            .style(Style::default().bg(self.theme_panel_bg_color()));
        f.render_widget(paragraph, area);
    }

    pub(super) fn draw_progress(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let current_duration = app
            .current_track()
            .and_then(|track| track.duration_secs)
            .unwrap_or(0);

        let ratio = if current_duration > 0 {
            (app.playback_state().position_secs as f64 / current_duration as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let pos = app.playback_state().position_secs;
        let label = format!(
            "{} / {}",
            Self::fmt_duration(pos),
            Self::fmt_duration(current_duration)
        );

        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(2)])
            .split(area);

        let label_width = sections[0].width.saturating_sub(2) as usize;
        let label = Self::fixed_width_text(&label, label_width);

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Progress ")
                    .style(Style::default().bg(self.theme_panel_bg_color())),
            )
            .gauge_style(Style::default().fg(self.theme_progress_color()))
            .ratio(ratio)
            .label(label);

        let remain = (current_duration - pos).max(0);
        let info = vec![
            Line::from(format!("Remaining: {}", Self::fmt_duration(remain))),
            Line::from(self.next_up_line(app)),
        ];
        let card = Paragraph::new(info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Metrics ")
                    .style(Style::default().bg(self.theme_panel_bg_color()))
                    .border_style(Style::default().fg(self.theme_dim_color())),
            )
            .style(Style::default().bg(self.theme_panel_bg_color()));

        f.render_widget(gauge, sections[0]);
        f.render_widget(card, sections[1]);
    }

    pub(super) fn draw_edit_tag_popup(&self, f: &mut ratatui::Frame<'_>) {
        let area = f.area();
        let popup_width = 54u16.min(area.width.saturating_sub(4));
        let popup_height = 7u16;
        let x = area.x + area.width.saturating_sub(popup_width) / 2;
        let y = area.y + area.height.saturating_sub(popup_height) / 2;
        let popup_area = Rect::new(x, y, popup_width, popup_height);

        let field_names = ["Title ", "Artist", "Album "];
        let lines: Vec<Line<'_>> = (0..3)
            .map(|i| {
                let cursor = if self.edit_tag_field == i { "_" } else { " " };
                let prefix = if self.edit_tag_field == i { "> " } else { "  " };
                let value = &self.edit_tag_inputs[i];
                Line::from(vec![
                    Span::styled(
                        format!("{}{}: ", prefix, field_names[i]),
                        if self.edit_tag_field == i {
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::Gray)
                        },
                    ),
                    Span::raw(format!("{}{}", value, cursor)),
                ])
            })
            .collect();

        let track_id_label = self
            .edit_tag_track_id
            .map(|id| format!(" #{} ", id))
            .unwrap_or_default();

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Edit Tags{}", track_id_label))
                .title_bottom(" [Tab] next  [Enter] save  [Esc] cancel ")
                .border_style(Style::default().fg(Color::Cyan)),
        );

        f.render_widget(Clear, popup_area);
        f.render_widget(paragraph, popup_area);
    }

    pub(super) fn draw_statusbar(&self, f: &mut ratatui::Frame<'_>, area: Rect) {
        let text = match self.mode {
            InputMode::Normal => {
                format!(
                    " {} | Up/Down Move | Enter Play | l Playlist | s Search | q Quit ",
                    self.status
                )
            }
            InputMode::Search => {
                format!(
                    " /{}_ | Up/Down Move | Enter Play | Esc Exit ",
                    self.search_input
                )
            }
            InputMode::PlaylistModal => {
                format!(
                    " {} | Up/Down Move | Enter Open/Play/Create | d Remove | r Rename | Esc Back/Close ",
                    self.status
                )
            }
            InputMode::EditTag => {
                let field_name = ["Title", "Artist", "Album"][self.edit_tag_field];
                let value = &self.edit_tag_inputs[self.edit_tag_field];
                format!(
                    " Edit [{}]: {}_ | Tab Next | Enter Save | Esc Cancel ",
                    field_name, value
                )
            }
        };
        let p = Paragraph::new(text).style(
            Style::default()
                .fg(self.theme_status_color())
                .bg(self.theme_panel_alt_bg_color()),
        );
        f.render_widget(p, area);
    }

    pub(super) fn next_up_line(&self, app: &App) -> String {
        let queue = app.queue_ids();
        if queue.is_empty() {
            return String::from("Up Next: (queue empty)");
        }

        let current_id = app.playback_state().current_track_id;
        let next_id = if let Some(curr) = current_id {
            queue
                .iter()
                .position(|id| *id == curr)
                .and_then(|idx| queue.get(idx + 1).copied())
                .or_else(|| queue.first().copied())
        } else {
            queue.first().copied()
        };

        if let Some(id) = next_id
            && let Some(track) = app.track_by_id(id)
        {
            return format!(
                "Up Next: {} - {}",
                track.artist.as_deref().unwrap_or("Unknown Artist"),
                track.title
            );
        }
        String::from("Up Next: (unknown)")
    }

    pub(super) fn fmt_duration(secs: i64) -> String {
        let s = secs.max(0);
        format!("{:02}:{:02}", s / 60, s % 60)
    }

    pub(super) fn fixed_width_text(text: &str, width: usize) -> String {
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
