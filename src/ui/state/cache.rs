use super::*;

impl UiState {
    fn is_filtering_library(&self) -> bool {
        self.mode == InputMode::Search
    }

    pub fn visible_track_ids(&mut self, app: &App) -> &[i64] {
        self.refresh_library_cache(app);
        &self.library_cached_track_ids
    }

    pub fn selected_track_id(&mut self, app: &App) -> Option<i64> {
        self.refresh_library_cache(app);
        if self.library_cached_track_ids.is_empty() {
            return None;
        }
        self.library_cached_track_ids
            .get(min(
                self.library_selected,
                self.library_cached_track_ids.len() - 1,
            ))
            .copied()
    }

    pub fn library_rows_for_width(&mut self, app: &App, width: usize) -> &[String] {
        self.refresh_library_cache(app);
        if self.library_render_width != width
            || self.library_cached_render_rows.len() != self.library_cached_rows.len()
        {
            self.library_cached_render_rows = self
                .library_cached_rows
                .iter()
                .map(|row| Self::fixed_width_cell(row, width))
                .collect();
            self.library_render_width = width;
        }
        &self.library_cached_render_rows
    }

    pub fn queue_track_ids(&mut self, app: &App) -> &[i64] {
        self.refresh_queue_cache(app);
        &self.queue_cached_track_ids
    }

    pub fn queue_rows_for_width(&mut self, app: &App, width: usize) -> &[String] {
        self.refresh_queue_cache(app);
        if self.queue_render_width != width
            || self.queue_cached_render_rows.len() != self.queue_cached_rows.len()
        {
            self.queue_cached_render_rows = self
                .queue_cached_rows
                .iter()
                .map(|row| Self::fixed_width_cell(row, width))
                .collect();
            self.queue_render_width = width;
        }
        &self.queue_cached_render_rows
    }

    pub(super) fn refresh_library_cache(&mut self, app: &App) {
        let tracks_version = app.tracks_version();
        let current_track_id = app.playback_state().current_track_id;
        if self.library_cache_tracks_version == tracks_version
            && self.library_cache_query == self.search_input
            && self.library_cache_mode == self.mode
            && self.library_cache_current_track_id == current_track_id
        {
            return;
        }

        self.library_cached_track_ids.clear();
        self.library_cached_rows.clear();

        if self.is_filtering_library() {
            self.search_warning = Self::search_warning_for_query(&self.search_input);
            let ids = app
                .search_track_ids_action(&self.search_input)
                .unwrap_or_default();
            self.library_cached_track_ids.reserve(ids.len());
            self.library_cached_rows.reserve(ids.len());
            for track_id in ids {
                if let Some(track) = app.track_by_id(track_id) {
                    self.library_cached_track_ids.push(track.id);
                    let marker = if Some(track.id) == current_track_id {
                        ">"
                    } else {
                        " "
                    };
                    let favorite = if track.favorite { "*" } else { " " };
                    self.library_cached_rows.push(format!(
                        "{}{} #{:04} {} - {}",
                        marker,
                        favorite,
                        track.id,
                        track.artist.as_deref().unwrap_or("Unknown Artist"),
                        track.title
                    ));
                }
            }
        } else {
            self.search_warning = None;
            let tracks = app.tracks();
            self.library_cached_track_ids.reserve(tracks.len());
            self.library_cached_rows.reserve(tracks.len());
            for track in tracks {
                self.library_cached_track_ids.push(track.id);
                let marker = if Some(track.id) == current_track_id {
                    ">"
                } else {
                    " "
                };
                let favorite = if track.favorite { "*" } else { " " };
                self.library_cached_rows.push(format!(
                    "{}{} #{:04} {} - {}",
                    marker,
                    favorite,
                    track.id,
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                    track.title
                ));
            }
        }

        self.library_cache_tracks_version = tracks_version;
        self.library_cache_query = self.search_input.clone();
        self.library_cache_mode = self.mode;
        self.library_cache_current_track_id = current_track_id;
        self.library_render_width = 0;
        self.library_cached_render_rows.clear();
    }

    pub(super) fn invalidate_library_cache(&mut self) {
        self.library_cache_tracks_version = 0;
        self.library_cache_query.clear();
        self.library_cache_current_track_id = None;
        self.library_cached_track_ids.clear();
        self.library_cached_rows.clear();
        self.library_render_width = 0;
        self.library_cached_render_rows.clear();
    }

    pub(super) fn refresh_queue_cache(&mut self, app: &App) {
        let queue_version = app.queue_version();
        let tracks_version = app.tracks_version();
        let current_track_id = app.playback_state().current_track_id;
        if self.queue_cache_version == queue_version
            && self.queue_cache_tracks_version == tracks_version
            && self.queue_cache_current_track_id == current_track_id
        {
            return;
        }

        self.queue_cached_track_ids.clear();
        self.queue_cached_rows.clear();

        let queue_ids = app.queue_ids();
        self.queue_cached_track_ids.reserve(queue_ids.len());
        self.queue_cached_rows.reserve(queue_ids.len());
        for (idx, track_id) in queue_ids.iter().copied().enumerate() {
            if let Some(track) = app.track_by_id(track_id) {
                self.queue_cached_track_ids.push(track.id);
                let marker = if Some(track.id) == current_track_id {
                    ">"
                } else {
                    " "
                };
                self.queue_cached_rows.push(format!(
                    "{} {:02}. {} - {}",
                    marker,
                    idx + 1,
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                    track.title
                ));
            }
        }

        self.queue_cache_version = queue_version;
        self.queue_cache_tracks_version = tracks_version;
        self.queue_cache_current_track_id = current_track_id;
        self.queue_render_width = 0;
        self.queue_cached_render_rows.clear();
    }

    pub fn cached_track_seed(&mut self, app: &App) -> u64 {
        let current_track_id = app.playback_state().current_track_id;
        let tracks_version = app.tracks_version();
        if self.seed_cache_valid
            && self.seed_cached_track_id == current_track_id
            && self.seed_cached_tracks_version == tracks_version
        {
            return self.seed_cached_value;
        }
        let mut hasher = DefaultHasher::new();
        if let Some(track) = app.current_track() {
            track.id.hash(&mut hasher);
            track.path.hash(&mut hasher);
            track.title.hash(&mut hasher);
            track.artist.hash(&mut hasher);
            track.album.hash(&mut hasher);
            track.duration_secs.hash(&mut hasher);
        } else {
            0u64.hash(&mut hasher);
        }
        let value = hasher.finish();
        self.seed_cache_valid = true;
        self.seed_cached_track_id = current_track_id;
        self.seed_cached_tracks_version = tracks_version;
        self.seed_cached_value = value;
        value
    }

    fn fixed_width_cell(text: &str, width: usize) -> String {
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
