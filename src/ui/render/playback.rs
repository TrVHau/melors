use super::*;

const NORMAL_STATUS_HINT: &str = "Up/Down Move | Enter Play | l Playlist | s Search | q Quit";
const SEARCH_STATUS_HINT: &str = "Up/Down Move | Enter Play | Esc Exit";
const PLAYLIST_STATUS_HINT: &str =
    "Up/Down Move | Enter Open/Create | d Remove | r Rename | x Delete | Esc Back/Close";
const EDIT_TAG_STATUS_HINT: &str = "Tab Next | Enter Save | Esc Cancel";

impl UiState {
    fn current_track_label(&self, app: &App) -> String {
        app.current_track()
            .map(|track| {
                format!(
                    "{} - {}",
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                    track.title
                )
            })
            .unwrap_or_else(|| String::from("Track: (none)"))
    }

    fn playback_flags_line(&self, app: &App) -> String {
        format!(
            "Repeat={} Shuffle={}",
            app.playback_state().repeat_mode,
            if app.playback_state().shuffle_enabled {
                "On"
            } else {
                "Off"
            }
        )
    }

    fn volume_line(&self, app: &App) -> String {
        format!("Vol {}%", app.volume_percent())
    }

    fn now_playing_lines(&self, app: &App) -> Vec<Line<'static>> {
        let mut lines = vec![
            Line::from(self.current_track_label(app)),
            Line::from(self.playback_flags_line(app)),
        ];

        if app.current_track().is_none() {
            lines.push(Line::from(self.next_up_line(app)));
        }

        lines.push(Line::from(self.volume_line(app)));
        lines
    }

    fn now_playing_border_style(&self, app: &App) -> Style {
        if app.is_actively_playing() {
            Style::default().fg(self.theme_library_color())
        } else if app.current_track().is_some() {
            Style::default().fg(self.theme_queue_color())
        } else {
            Style::default().fg(self.theme_dim_color())
        }
    }

    fn progress_label(&self, app: &App, current_duration: i64) -> String {
        let pos = app.playback_state().position_secs;
        format!(
            "{} / {}",
            Self::fmt_duration(pos),
            Self::fmt_duration(current_duration)
        )
    }

    fn progress_ratio(&self, app: &App, current_duration: i64) -> f64 {
        if current_duration > 0 {
            (app.playback_state().position_secs as f64 / current_duration as f64).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    fn statusbar_text(&self) -> String {
        match self.mode {
            InputMode::Normal => format!(" {} | {} ", self.status, NORMAL_STATUS_HINT),
            InputMode::Search => format!(" /{}_ | {} ", self.search_input, SEARCH_STATUS_HINT),
            InputMode::PlaylistModal => {
                format!(" {} | {} ", self.status, PLAYLIST_STATUS_HINT)
            }
            InputMode::EditTag => {
                let field_name = ["Title", "Artist", "Album"][self.edit_tag_field];
                let value = &self.edit_tag_inputs[self.edit_tag_field];
                format!(" Edit [{}]: {}_ | {} ", field_name, value, EDIT_TAG_STATUS_HINT)
            }
        }
    }

    pub(super) fn draw_now_playing(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let panel_bg = self.theme_playback_panel_bg_color();
        let paragraph = Paragraph::new(self.now_playing_lines(app))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Now Playing ")
                    .style(Style::default().bg(panel_bg))
                    .border_style(self.now_playing_border_style(app)),
            )
            .style(Style::default().bg(panel_bg));
        f.render_widget(paragraph, area);
    }

    pub(super) fn draw_progress(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let current_duration = app
            .current_track()
            .and_then(|track| track.duration_secs)
            .unwrap_or(0);

        let ratio = self.progress_ratio(app, current_duration);
        let label = self.progress_label(app, current_duration);
        let panel_bg = self.theme_progress_panel_bg_color();

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
                    .style(Style::default().bg(panel_bg)),
            )
            .gauge_style(Style::default().fg(self.theme_progress_color()))
            .ratio(ratio)
            .label(label);

        let remain = (current_duration - app.playback_state().position_secs).max(0);
        let info = vec![
            Line::from(format!("Remaining: {}", Self::fmt_duration(remain))),
            Line::from(self.next_up_line(app)),
        ];
        let card = Paragraph::new(info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Metrics ")
                    .style(Style::default().bg(panel_bg))
                    .border_style(Style::default().fg(self.theme_dim_color())),
            )
            .style(Style::default().bg(panel_bg));

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
        let popup_bg = self.theme_modal_bg_color();
        let active_color = self.theme_popup_active_color();
        let inactive_color = self.theme_popup_inactive_color();

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
                                .fg(active_color)
                                .bg(popup_bg)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(inactive_color).bg(popup_bg)
                        },
                    ),
                    Span::styled(
                        format!("{}{}", value, cursor),
                        Style::default().fg(self.theme_status_color()).bg(popup_bg),
                    ),
                ])
            })
            .collect();

        let track_id_label = self
            .edit_tag_track_id
            .map(|id| format!(" #{} ", id))
            .unwrap_or_default();

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Edit Tags{}", track_id_label))
                    .title_bottom(" [Tab] next  [Enter] save  [Esc] cancel ")
                    .style(Style::default().bg(popup_bg))
                    .border_style(Style::default().fg(self.theme_popup_border_color())),
            )
            .style(Style::default().bg(popup_bg));

        f.render_widget(Clear, popup_area);
        f.render_widget(paragraph, popup_area);
    }

    pub(super) fn draw_statusbar(&self, f: &mut ratatui::Frame<'_>, area: Rect) {
        let p = Paragraph::new(self.statusbar_text()).style(
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
