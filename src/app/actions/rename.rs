use super::*;

impl App {
    pub fn write_track_tags(
        &mut self,
        track_id: i64,
        title: &str,
        artist: &str,
        album: &str,
    ) -> Result<()> {
        let (old_path, new_path) = match self.track_by_id(track_id) {
            Some(t) => {
                let old_path = t.path.clone();
                let ext = old_path.extension().and_then(|e| e.to_str()).unwrap_or("");
                let safe_stem = build_filename_stem(title, artist);
                let new_filename = if ext.is_empty() {
                    safe_stem
                } else {
                    format!("{}.{}", safe_stem, ext)
                };
                let new_path = old_path
                    .parent()
                    .unwrap_or(Path::new("."))
                    .join(&new_filename);
                (old_path, new_path)
            }
            None => return Ok(()),
        };

        if new_path != old_path {
            std::fs::rename(&old_path, &new_path)?;
        }

        crate::services::metadata::write_tag(
            &new_path,
            title,
            if artist.is_empty() {
                None
            } else {
                Some(artist)
            },
            if album.is_empty() { None } else { Some(album) },
        )?;
        self.storage
            .rename_track(track_id, title, &new_path.to_string_lossy())?;
        self.storage.rename_artist(track_id, artist)?;
        self.storage.rename_album(track_id, album)?;
        self.reload_session_state()
    }
}

fn sanitize_filename_part(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        let mapped = match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        };
        out.push(mapped);
    }

    out.trim().trim_end_matches('.').to_string()
}

fn build_filename_stem(title: &str, artist: &str) -> String {
    let safe_title = sanitize_filename_part(title);
    let safe_artist = sanitize_filename_part(artist);

    let combined = if safe_artist.is_empty() {
        safe_title
    } else if safe_title.is_empty() {
        safe_artist
    } else {
        format!("{} - {}", safe_artist, safe_title)
    };

    let trimmed = combined.trim().trim_end_matches('.');
    if trimmed.is_empty() {
        String::from("untitled")
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{build_filename_stem, sanitize_filename_part};

    #[test]
    fn sanitize_filename_part_replaces_invalid_chars() {
        let value = sanitize_filename_part("A/B:C*D?E\"F<G>H|I");
        assert_eq!(value, "A_B_C_D_E_F_G_H_I");
    }

    #[test]
    fn build_filename_stem_uses_artist_and_title() {
        let value = build_filename_stem("Song Name", "Artist Name");
        assert_eq!(value, "Artist Name - Song Name");
    }

    #[test]
    fn build_filename_stem_falls_back_when_empty() {
        let value = build_filename_stem("...   ", "");
        assert_eq!(value, "untitled");
    }
}
