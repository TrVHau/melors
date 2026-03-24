use super::*;

impl UiState {
    fn fit_modal_text(value: &str, width: usize) -> String {
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

    fn playlist_item_row(
        &self,
        app: &App,
        idx: usize,
        item: &crate::services::storage::PlaylistItem,
        content_width: usize,
    ) -> ListItem<'static> {
        let label = if item.is_missing {
            format!(
                "{:02}. [missing] {}",
                idx + 1,
                item.original_path.clone().unwrap_or_default()
            )
        } else if let Some(track_id) = item.track_id {
            format!("{:02}. {}", idx + 1, self.track_label_by_id(app, track_id))
        } else {
            format!(
                "{:02}. [missing] {}",
                idx + 1,
                item.original_path.clone().unwrap_or_default()
            )
        };

        ListItem::new(Self::fit_modal_text(&label, content_width))
    }

    fn playlist_items_view(
        &self,
        app: &App,
        playlists: &[crate::services::storage::Playlist],
        items: &[crate::services::storage::PlaylistItem],
        content_width: usize,
    ) -> (
        String,
        Vec<ListItem<'static>>,
        Option<usize>,
        Color,
        &'static str,
    ) {
        let selected_name = playlists
            .get(self.playlist_selected)
            .map(|p| p.name.as_str())
            .unwrap_or("(no playlist)");

        let rows = if items.is_empty() {
            vec![ListItem::new("(playlist empty)")]
        } else {
            items
                .iter()
                .enumerate()
                .map(|(idx, item)| self.playlist_item_row(app, idx, item, content_width))
                .collect()
        };

        (
            format!(" Playlist: {} ({} items) ", selected_name, items.len()),
            rows,
            (!items.is_empty()).then_some(self.playlist_item_selected),
            self.theme_queue_color(),
            " Up/Down move | Enter play | d remove | Esc back ",
        )
    }

    fn rename_playlist_view(
        &self,
        content_width: usize,
    ) -> (
        String,
        Vec<ListItem<'static>>,
        Option<usize>,
        Color,
        &'static str,
    ) {
        let display = format!("Name: {}_", self.playlist_rename_input);
        (
            String::from(" Rename Playlist "),
            vec![ListItem::new(Self::fit_modal_text(&display, content_width))],
            None,
            self.theme_library_color(),
            " Type name | Enter save | Esc cancel ",
        )
    }

    fn create_playlist_view(
        &self,
        content_width: usize,
    ) -> (
        String,
        Vec<ListItem<'static>>,
        Option<usize>,
        Color,
        &'static str,
    ) {
        let display = format!("Name: {}_", self.playlist_rename_input);
        (
            String::from(" Create Playlist "),
            vec![ListItem::new(Self::fit_modal_text(&display, content_width))],
            None,
            self.theme_library_color(),
            " Type name | Enter create | Esc cancel ",
        )
    }

    fn confirm_delete_playlist_view(
        &self,
        playlists: &[crate::services::storage::Playlist],
        content_width: usize,
    ) -> (
        String,
        Vec<ListItem<'static>>,
        Option<usize>,
        Color,
        &'static str,
    ) {
        let playlist_name = playlists
            .get(self.playlist_selected)
            .map(|playlist| playlist.name.as_str())
            .unwrap_or("(missing)");
        let prompt = format!("Delete playlist: {}", playlist_name);
        (
            String::from(" Confirm Delete "),
            vec![ListItem::new(Self::fit_modal_text(&prompt, content_width))],
            None,
            self.theme_modal_warning_color(),
            " Enter/x confirm | Esc cancel ",
        )
    }

    fn playlists_browser_view(
        &self,
        app: &App,
        playlists: &[crate::services::storage::Playlist],
        content_width: usize,
    ) -> (
        String,
        Vec<ListItem<'static>>,
        Option<usize>,
        Color,
        &'static str,
    ) {
        let mut rows = if playlists.is_empty() {
            Vec::new()
        } else {
            playlists
                .iter()
                .map(|playlist| {
                    let raw = format!("#{} {}", playlist.id, playlist.name);
                    ListItem::new(Self::fit_modal_text(&raw, content_width))
                })
                .collect()
        };
        rows.push(ListItem::new(Self::fit_modal_text(
            "+ New playlist",
            content_width,
        )));

        let picker_mode = self.playlist_add_track_id.is_some();
        let title = if let Some(track_id) = self.playlist_add_track_id {
            let track_label = self.track_label_by_id(app, track_id);
            format!(
                " Add: {} ",
                Self::fit_modal_text(&track_label, content_width.saturating_sub(6))
            )
        } else {
            format!(" Playlists ({}) ", playlists.len())
        };
        let helper = if picker_mode {
            " Up/Down move | Enter add/create+add | Esc cancel "
        } else {
            " Up/Down move | Enter open/create | r rename | x delete | Esc close "
        };

        (
            title,
            rows,
            Some(self.playlist_selected.min(playlists.len())),
            self.theme_library_color(),
            helper,
        )
    }

    pub(super) fn draw_playlist_modal(
        &mut self,
        f: &mut ratatui::Frame<'_>,
        area: Rect,
        app: &App,
    ) {
        if !self.playlist_modal_visible {
            return;
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
        let (title, rows, selected, border_color, helper) = match self.playlist_modal_mode {
            PlaylistModalMode::BrowseItems => {
                self.playlist_items_view(app, &playlists, &items, content_width)
            }
            PlaylistModalMode::CreatePlaylist => self.create_playlist_view(content_width),
            PlaylistModalMode::RenamePlaylist => self.rename_playlist_view(content_width),
            PlaylistModalMode::ConfirmDeletePlaylist => {
                self.confirm_delete_playlist_view(&playlists, content_width)
            }
            PlaylistModalMode::BrowsePlaylists => {
                self.playlists_browser_view(app, &playlists, content_width)
            }
        };

        let mut state = ListState::default();
        state.select(selected);
        let modal_bg = self.theme_modal_bg_color();
        let highlight_color = self.theme_modal_highlight_color();
        let border_color = if self.playlist_modal_mode == PlaylistModalMode::ConfirmDeletePlaylist {
            border_color
        } else {
            self.theme_modal_border_color()
        };

        let list = List::new(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_bottom(helper)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().bg(modal_bg)),
            )
            .style(Style::default().bg(modal_bg))
            .highlight_style(
                Style::default()
                    .fg(highlight_color)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        f.render_stateful_widget(list, vertical[0], &mut state);
    }
}
