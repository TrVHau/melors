use super::*;

impl UiState {
    pub(super) fn draw_library(&mut self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let content_width = area.width.saturating_sub(4) as usize;
        let rows_len = self.library_rows_for_width(app, content_width).len();
        if rows_len == 0 {
            self.library_selected = 0;
        } else {
            self.library_selected = min(self.library_selected, rows_len - 1);
        }
        let selected = if rows_len == 0 {
            None
        } else {
            Some(self.library_selected)
        };

        let is_active = matches!(self.focus, FocusPanel::Library)
            || matches!(
                self.mode,
                InputMode::Search | InputMode::EditTag | InputMode::PlaylistModal
            );
        let mode_suffix = match self.mode {
            InputMode::Search => " [/]",
            InputMode::EditTag => " [edit]",
            InputMode::PlaylistModal => " [list]",
            InputMode::Normal => "",
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Library {}{}", rows_len, mode_suffix))
            .border_style(if is_active {
                Style::default().fg(self.theme_library_color())
            } else {
                Style::default().fg(self.theme_dim_color())
            })
            .style(Style::default().bg(self.theme_library_panel_bg_color(is_active)));

        let current_id = app.playback_state().current_track_id;
        let rows: Vec<String> = self.library_rows_for_width(app, content_width).to_vec();
        let items: Vec<ListItem<'_>> = rows
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let id = self.library_cached_track_ids.get(i).copied();
                if id.is_some() && id == current_id {
                    ListItem::new(row.as_str()).style(
                        Style::default()
                            .fg(self.theme_now_playing_row_color())
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(row.as_str())
                }
            })
            .collect();

        let mut state = ListState::default();
        state.select(selected);

        let list = List::new(items)
            .block(block)
            .style(Style::default().bg(self.theme_library_panel_bg_color(is_active)))
            .highlight_style(
                Style::default()
                    .bg(self.theme_library_color())
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        f.render_stateful_widget(list, area, &mut state);
    }
}
