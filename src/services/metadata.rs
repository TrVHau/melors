use std::path::Path;

use id3::TagLike;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration_secs: Option<i64>,
}

pub fn write_tag(
    path: &Path,
    title: &str,
    artist: Option<&str>,
    album: Option<&str>,
) -> anyhow::Result<()> {
    let mut tag = id3::Tag::read_from_path(path).unwrap_or_default();
    tag.set_title(title);
    match artist {
        Some(a) => tag.set_artist(a),
        None => tag.remove_artist(),
    }
    match album {
        Some(a) => tag.set_album(a),
        None => tag.remove_album(),
    }
    tag.write_to_path(path, id3::Version::Id3v24)?;
    Ok(())
}

pub fn read_metadata(path: &Path) -> Metadata {
    let fallback_title = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown Title")
        .to_string();

    match id3::Tag::read_from_path(path) {
        Ok(tag) => {
            let title = tag.title().map(str::to_owned).unwrap_or(fallback_title);
            let artist = tag.artist().map(str::to_owned);
            let album = tag.album().map(str::to_owned);
            let duration_secs = tag.duration().map(i64::from);
            Metadata {
                title,
                artist,
                album,
                duration_secs,
            }
        }
        Err(_) => Metadata {
            title: fallback_title,
            artist: None,
            album: None,
            duration_secs: None,
        },
    }
}
