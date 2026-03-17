use super::*;

impl UiState {
    pub(super) fn draw_playlist_modal(&mut self, f: &mut ratatui::Frame<'_>, app: &App) {
        if !self.playlist_modal_visible {
            return;
        }

        let area = f.area();
        let popup_width = 84u16.min(area.width.saturating_sub(4));
        let popup_height = 22u16.min(area.height.saturating_sub(4));
        let x = area.x + area.width.saturating_sub(popup_width) / 2;
        let y = area.y + area.height.saturating_sub(popup_height) / 2;
        let popup_area = Rect::new(x, y, popup_width, popup_height);

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(1)])
            .split(popup_area);

        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
            .split(vertical[0]);

        let playlists = app.list_playlists_action().unwrap_or_default();
        if playlists.is_empty() {
            self.playlist_selected = 0;
        } else {
            self.playlist_selected = self.playlist_selected.min(playlists.len() - 1);
        }
        let playlist_items: Vec<ListItem<'_>> = if playlists.is_empty() {
            vec![ListItem::new("(no playlists)")]
        } else {
            playlists
                .iter()
                .map(|p| ListItem::new(format!("#{} {}", p.id, p.name)))
                .collect()
        };

        let mut left_state = ListState::default();
        left_state.select(if playlists.is_empty() {
            None
        } else {
            Some(self.playlist_selected)
        });

        let left = List::new(playlist_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Playlists ")
                    .border_style(
                        if self.playlist_modal_mode == PlaylistModalMode::BrowsePlaylists {
                            Style::default().fg(Color::Cyan)
                        } else {
                            Style::default().fg(Color::Gray)
                        },
                    ),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        let selected_playlist_id = playlists.get(self.playlist_selected).map(|p| p.id);
        let items = selected_playlist_id
            .and_then(|id| app.list_playlist_items_action(id).ok())
            .unwrap_or_default();

        if items.is_empty() {
            self.playlist_item_selected = 0;
        } else {
            self.playlist_item_selected = self.playlist_item_selected.min(items.len() - 1);
        }

        let right_rows: Vec<ListItem<'_>> = if items.is_empty() {
            vec![ListItem::new("(no playlist items)")]
        } else {
            items
                .iter()
                .enumerate()
                .map(|(idx, item)| {
                    let label = if item.is_missing {
                        format!(
                            "{:02}. [missing] {}",
                            idx + 1,
                            item.original_path.clone().unwrap_or_default()
                        )
                    } else if let Some(track_id) = item.track_id {
                        if let Some(track) = app.track_by_id(track_id) {
                            format!(
                                "{:02}. {} - {}",
                                idx + 1,
                                track.artist.as_deref().unwrap_or("Unknown Artist"),
                                track.title
                            )
                        } else {
                            format!("{:02}. [missing] #{}", idx + 1, track_id)
                        }
                    } else {
                        format!(
                            "{:02}. [missing] {}",
                            idx + 1,
                            item.original_path.clone().unwrap_or_default()
                        )
                    };
                    ListItem::new(label)
                })
                .collect()
        };

        let mut right_state = ListState::default();
        right_state.select(if items.is_empty() {
            None
        } else {
            Some(self.playlist_item_selected)
        });

        let right = List::new(right_rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Playlist Items ")
                    .border_style(
                        if self.playlist_modal_mode == PlaylistModalMode::BrowseItems {
                            Style::default().fg(Color::Cyan)
                        } else {
                            Style::default().fg(Color::Gray)
                        },
                    ),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        f.render_widget(Clear, popup_area);
        f.render_stateful_widget(left, cols[0], &mut left_state);
        f.render_stateful_widget(right, cols[1], &mut right_state);

        let help = Paragraph::new(
            "[Esc/l] close  [Enter] open/play  [b] back  [c] create  [R] rename  [d] delete  [a] add  [x] remove  [Shift+Up/Down] move",
        )
        .style(Style::default().fg(Color::Gray));
        f.render_widget(help, vertical[1]);
    }
}
