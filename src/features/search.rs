use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::core::model::Track;

pub fn search_tracks<'a>(tracks: &'a [Track], keyword: &str) -> Vec<&'a Track> {
    if keyword.trim().is_empty() {
        return tracks.iter().collect();
    }

    let matcher = SkimMatcherV2::default();
    let mut scored: Vec<(&Track, i64)> = tracks
        .iter()
        .filter_map(|track| {
            let haystack = format!(
                "{} {} {}",
                track.title,
                track.artist.as_deref().unwrap_or_default(),
                track.album.as_deref().unwrap_or_default()
            );
            matcher
                .fuzzy_match(&haystack, keyword)
                .map(|score| (track, score))
        })
        .collect();

    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored.into_iter().map(|(track, _)| track).collect()
}
