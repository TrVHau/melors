use super::*;

impl UiState {
    pub(super) fn draw_playlist_modal(
        &mut self,
        f: &mut ratatui::Frame<'_>,
        area: Rect,
        app: &App,
    ) {
        if !self.playlist_modal_visible {
            return;
        }

        fn fit_text(value: &str, width: usize) -> String {
            if width == 0 {
                return String::new();
            }
            let mut out: String = value.chars().take(width).collect();
            if value.chars().count() > width && width > 2 {
                out = value.chars().take(width - 2).collect();
                out.push_str("..");
            }
            out
        }

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6)])
            .split(area);

        let split = if area.width < 72 {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
                .split(vertical[0])
        } else {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(vertical[0])
        };
        let left_area = split[0];
        let right_area = split[1];

        let playlists = app.list_playlists_action().unwrap_or_default();
        if playlists.is_empty() {
            self.playlist_selected = 0;
        } else {
            self.playlist_selected = self.playlist_selected.min(playlists.len() - 1);
        }
        let playlist_items: Vec<ListItem<'_>> = if playlists.is_empty() {
            vec![ListItem::new("(no playlists)")]
        } else {
            let width = left_area.width.saturating_sub(8) as usize;
            playlists
                .iter()
                .map(|p| {
                    let raw = format!("#{} {}", p.id, p.name);
                    ListItem::new(fit_text(&raw, width))
                })
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
                    .title(format!(" Playlists ({}) ", playlists.len()))
                    .border_style(
                        if matches!(
                            self.playlist_modal_mode,
                            PlaylistModalMode::BrowsePlaylists
                                | PlaylistModalMode::CreatePlaylistName
                                | PlaylistModalMode::RenamePlaylistName
                        ) {
                            Style::default().fg(self.theme_library_color())
                        } else {
                            Style::default().fg(self.theme_dim_color())
                        },
                    )
                    .style(Style::default().bg(
                        if matches!(
                            self.playlist_modal_mode,
                            PlaylistModalMode::BrowsePlaylists
                                | PlaylistModalMode::CreatePlaylistName
                                | PlaylistModalMode::RenamePlaylistName
                        ) {
                            self.theme_panel_alt_bg_color()
                        } else {
                            self.theme_panel_bg_color()
                        },
                    )),
            )
            .style(Style::default().bg(
                if matches!(
                    self.playlist_modal_mode,
                    PlaylistModalMode::BrowsePlaylists
                        | PlaylistModalMode::CreatePlaylistName
                        | PlaylistModalMode::RenamePlaylistName
                ) {
                    self.theme_panel_alt_bg_color()
                } else {
                    self.theme_panel_bg_color()
                },
            ))
            .highlight_style(
                Style::default()
                    .fg(self.theme_header_color())
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
            let width = right_area.width.saturating_sub(8) as usize;
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
                    ListItem::new(fit_text(&label, width))
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
                    .title(format!(" Items ({}) ", items.len()))
                    .border_style(
                        if self.playlist_modal_mode == PlaylistModalMode::BrowseItems {
                            Style::default().fg(self.theme_queue_color())
                        } else {
                            Style::default().fg(self.theme_dim_color())
                        },
                    )
                    .style(Style::default().bg(
                        if self.playlist_modal_mode == PlaylistModalMode::BrowseItems {
                            self.theme_panel_alt_bg_color()
                        } else {
                            self.theme_panel_bg_color()
                        },
                    )),
            )
            .style(Style::default().bg(
                if self.playlist_modal_mode == PlaylistModalMode::BrowseItems {
                    self.theme_panel_alt_bg_color()
                } else {
                    self.theme_panel_bg_color()
                },
            ))
            .highlight_style(
                Style::default()
                    .fg(self.theme_header_color())
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        f.render_stateful_widget(left, left_area, &mut left_state);
        f.render_stateful_widget(right, right_area, &mut right_state);
    }
}
