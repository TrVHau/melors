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
            || matches!(self.mode, InputMode::Search | InputMode::Rename | InputMode::EditTag);
        let mode_suffix = match self.mode {
            InputMode::Search => " [/] ",
            InputMode::Rename => " [rename] ",
            InputMode::EditTag => " [edit tag] ",
            InputMode::Normal => " ",
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Library{mode_suffix}"))
            .border_style(if is_active {
                Style::default().fg(self.theme_library_color())
            } else {
                Style::default().fg(self.theme_dim_color())
            });

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
            .highlight_style(
                Style::default()
                    .bg(self.theme_library_color())
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        f.render_stateful_widget(list, area, &mut state);
    }

    pub(super) fn draw_queue(&mut self, f: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
        let content_width = area.width.saturating_sub(4) as usize;
        let queue_len = self.queue_rows_for_width(app, content_width).len();
        if queue_len == 0 {
            self.queue_selected = 0;
        } else {
            self.queue_selected = min(self.queue_selected, queue_len - 1);
        }
        let selected = if queue_len == 0 {
            None
        } else {
            Some(self.queue_selected)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Queue ")
            .border_style(if matches!(self.focus, FocusPanel::Queue) {
                Style::default().fg(self.theme_queue_color())
            } else {
                Style::default().fg(self.theme_dim_color())
            });

        let queue_rows = self.queue_rows_for_width(app, content_width).to_vec();
        let items: Vec<ListItem<'_>> = if queue_len == 0 {
            vec![ListItem::new("(queue empty)")]
        } else {
            queue_rows
                .iter()
                .map(|row| ListItem::new(row.as_str()))
                .collect()
        };

        let mut state = ListState::default();
        state.select(selected);

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(self.theme_queue_color())
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        f.render_stateful_widget(list, area, &mut state);
    }
}
