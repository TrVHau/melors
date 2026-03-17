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

        let playlists = app.list_playlists_action().unwrap_or_default();
        self.playlist_selected = self.playlist_selected.min(playlists.len());
        let selected_playlist_id = playlists.get(self.playlist_selected).map(|p| p.id);
        let items = selected_playlist_id
            .and_then(|id| app.list_playlist_items_action(id).ok())
            .unwrap_or_default();

        if items.is_empty() {
            self.playlist_item_selected = 0;
        } else {
            self.playlist_item_selected = self.playlist_item_selected.min(items.len() - 1);
        }

        let content_width = vertical[0].width.saturating_sub(8) as usize;
        let (title, rows, selected, border_color, helper) =
            if self.playlist_modal_mode == PlaylistModalMode::BrowseItems {
                let selected_name = playlists
                    .get(self.playlist_selected)
                    .map(|p| p.name.as_str())
                    .unwrap_or("(no playlist)");

                let rows: Vec<ListItem<'_>> = if items.is_empty() {
                    vec![ListItem::new("(playlist empty)")]
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
                            ListItem::new(fit_text(&label, content_width))
                        })
                        .collect()
                };

                (
                    format!(" Playlist: {} ({} items) ", selected_name, items.len()),
                    rows,
                    if items.is_empty() {
                        None
                    } else {
                        Some(self.playlist_item_selected)
                    },
                    self.theme_queue_color(),
                    " Up/Down move | Enter play | d remove | Esc back ",
                )
            } else if self.playlist_modal_mode == PlaylistModalMode::RenamePlaylist {
                let display = format!("Name: {}_", self.playlist_rename_input);
                (
                    String::from(" Rename Playlist "),
                    vec![ListItem::new(fit_text(&display, content_width))],
                    None,
                    self.theme_library_color(),
                    " Type name | Enter save | Esc cancel ",
                )
            } else {
                let mut rows: Vec<ListItem<'_>> = if playlists.is_empty() {
                    Vec::new()
                } else {
                    playlists
                        .iter()
                        .map(|p| {
                            let raw = format!("#{} {}", p.id, p.name);
                            ListItem::new(fit_text(&raw, content_width))
                        })
                        .collect()
                };
                rows.push(ListItem::new(fit_text("+ New playlist", content_width)));

                let picker_mode = self.playlist_add_track_id.is_some();
                let title = if let Some(track_id) = self.playlist_add_track_id {
                    let track_label = app
                        .track_by_id(track_id)
                        .map(|track| {
                            format!(
                                "{} - {}",
                                track.artist.as_deref().unwrap_or("Unknown Artist"),
                                track.title
                            )
                        })
                        .unwrap_or_else(|| format!("#{}", track_id));
                    format!(" Add: {} ", fit_text(&track_label, content_width.saturating_sub(6)))
                } else {
                    format!(" Playlists ({}) ", playlists.len())
                };
                let helper = if picker_mode {
                    " Up/Down move | Enter add/create+add | Esc cancel "
                } else {
                    " Up/Down move | Enter open/create | r rename | Esc close "
                };

                (
                    title,
                    rows,
                    Some(self.playlist_selected.min(playlists.len())),
                    self.theme_library_color(),
                    helper,
                )
            };

        let mut state = ListState::default();
        state.select(selected);

        let list = List::new(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_bottom(helper)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().bg(self.theme_panel_alt_bg_color())),
            )
            .style(Style::default().bg(self.theme_panel_alt_bg_color()))
            .highlight_style(
                Style::default()
                    .fg(self.theme_header_color())
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        f.render_stateful_widget(list, vertical[0], &mut state);
    }
}
