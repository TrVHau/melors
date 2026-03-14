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
            let title_score = matcher.fuzzy_match(&track.title, keyword);
            let artist_score = track
                .artist
                .as_deref()
                .and_then(|artist| matcher.fuzzy_match(artist, keyword));
            let album_score = track
                .album
                .as_deref()
                .and_then(|album| matcher.fuzzy_match(album, keyword));

            let score = title_score
                .into_iter()
                .chain(artist_score)
                .chain(album_score)
                .max();

            score.map(|value| (track, value))
        })
        .collect();

    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored.into_iter().map(|(track, _)| track).collect()
}
