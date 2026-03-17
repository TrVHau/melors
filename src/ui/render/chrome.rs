use super::*;

impl UiState {
    pub fn draw(&mut self, f: &mut ratatui::Frame<'_>, app: &App) {
        let area = f.area();
        let tiny = area.width < 90 || area.height < 24;
        let compact = area.width < 125 || area.height < 34;

        let root = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(if tiny { 10 } else { 14 }),
                Constraint::Length(1),
            ])
            .split(area);

        self.draw_header(f, root[0], app);

        if tiny {
            self.draw_tiny_layout(f, root[1], app);
        } else if compact {
            self.draw_compact_layout(f, root[1], app);
        } else {
            self.draw_full_layout(f, root[1], app);
        }

        self.draw_statusbar(f, root[2]);

        if self.mode == InputMode::EditTag {
            self.draw_edit_tag_popup(f);
        }
    }

    fn draw_tiny_layout(&mut self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
            .split(area);

        if self.mode == InputMode::PlaylistModal {
            self.draw_playlist_modal(f, rows[0], app);
        } else {
            self.draw_library(f, rows[0], app);
        }
        self.draw_now_playing(f, rows[1], app);
    }

    fn draw_compact_layout(&mut self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(66), Constraint::Percentage(34)])
            .split(area);

        let top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
            .split(rows[0]);

        if self.mode == InputMode::PlaylistModal {
            self.draw_playlist_modal(f, top[0], app);
        } else {
            self.draw_library(f, top[0], app);
        }
        self.draw_now_playing(f, top[1], app);

        if self.mode == InputMode::PlaylistModal {
            let bottom = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
                .split(rows[1]);
            self.draw_progress(f, bottom[0], app);
            self.draw_visualizer_panel(f, bottom[1], app);
        } else {
            let bottom = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(44), Constraint::Percentage(56)])
                .split(rows[1]);
            self.draw_now_playing(f, bottom[0], app);
            self.draw_progress(f, bottom[1], app);
        }
    }

    fn draw_full_layout(&mut self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        let top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(64), Constraint::Percentage(36)])
            .split(rows[0]);

        if self.mode == InputMode::PlaylistModal {
            self.draw_playlist_modal(f, top[0], app);
        } else {
            self.draw_library(f, top[0], app);
        }
        self.draw_now_playing(f, top[1], app);

        let bottom = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(36), Constraint::Percentage(64)])
            .split(rows[1]);
        self.draw_progress(f, bottom[0], app);
        self.draw_visualizer_panel(f, bottom[1], app);
    }

    pub(super) fn draw_header(&self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let mode = match self.mode {
            InputMode::Normal => "NORMAL",
            InputMode::Search => "SEARCH",
            InputMode::EditTag => "EDIT",
            InputMode::PlaylistModal => "PLAYLIST",
        };
        let current = app
            .current_track()
            .map(|track| {
                format!(
                    "{} - {}",
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                    track.title
                )
            })
            .unwrap_or_else(|| String::from("no track"));

        let text = format!(
            " melors [{mode}] | {} ",
            current
        );
        let p = Paragraph::new(text).style(
            Style::default()
                .fg(self.theme_header_color())
                .bg(self.theme_panel_bg_color()),
        );
        f.render_widget(p, area);
    }
}
