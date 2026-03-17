use super::*;

impl UiState {
    pub fn draw(&mut self, f: &mut ratatui::Frame<'_>, app: &App) {
        let root = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(8),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(f.area());

        let content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(32), Constraint::Length(36)])
            .split(root[2]);

        let bottom = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(root[3]);

        self.draw_header(f, root[0], app);
        self.draw_tabs(f, root[1]);
        self.draw_library(f, content[0], app);
        self.draw_queue(f, content[1], app);
        self.draw_now_playing(f, bottom[0], app);
        self.draw_visualizer_panel(f, bottom[1], app);
        self.draw_progress(f, root[4], app);
        self.draw_statusbar(f, root[5]);

        if self.mode == InputMode::EditTag {
            self.draw_edit_tag_popup(f);
        }
        if self.mode == InputMode::PlaylistModal {
            self.draw_playlist_modal(f, app);
        }
    }

    pub(super) fn draw_header(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let text = if let Some(track) = app.current_track() {
            let artist = track.artist.as_deref().unwrap_or("Unknown Artist");
            let album = track.album.as_deref().unwrap_or("");
            let icon = if app.is_actively_playing() { ">" } else { "||" };
            let album_part = if album.is_empty() {
                String::new()
            } else {
                format!("  [{}]", album)
            };
            format!(" {} {} - {}{}", icon, artist, track.title, album_part)
        } else {
            String::from("  melors  -  no track loaded")
        };

        let p = Paragraph::new(text).style(Style::default().fg(self.theme_header_color()));
        f.render_widget(p, area);
    }

    pub(super) fn draw_tabs(&self, f: &mut ratatui::Frame<'_>, area: Rect) {
        let tab_idx = if self.mode == InputMode::Search {
            1
        } else {
            match self.focus {
                FocusPanel::Queue => 2,
                _ => 0,
            }
        };
        let titles = vec![" Library ", " Search ", " Queue "];
        let is_focus = !matches!(self.focus, FocusPanel::Queue) || self.mode == InputMode::Search;
        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" melors ")
                    .border_style(if is_focus {
                        Style::default().fg(self.theme_accent_color())
                    } else {
                        Style::default().fg(self.theme_dim_color())
                    }),
            )
            .select(tab_idx)
            .style(Style::default().fg(self.theme_muted_color()))
            .highlight_style(
                Style::default()
                    .fg(self.theme_highlight_color())
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(tabs, area);
    }
}
