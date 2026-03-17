#![allow(dead_code)]

use super::*;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StructuredSearchFilter {
    pub artist: Option<String>,
    pub album: Option<String>,
    pub favorite: Option<bool>,
}

impl StructuredSearchFilter {
    fn has_any(&self) -> bool {
        self.artist.is_some() || self.album.is_some() || self.favorite.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchOrchestrationQuery {
    pub raw_query: String,
    pub structured_filters: StructuredSearchFilter,
    pub residual_term: Option<String>,
}

impl App {
    pub fn parse_search_orchestration_query(raw_query: &str) -> SearchOrchestrationQuery {
        parse_search_orchestration_query(raw_query)
    }

    pub fn search_track_ids_action(&self, raw_query: &str) -> Result<Vec<i64>> {
        let query = parse_search_orchestration_query(raw_query);
        let filters = &query.structured_filters;

        let filtered_ids = if filters.has_any() {
            self.storage.search_track_ids_with_filters(
                filters.artist.as_deref(),
                filters.album.as_deref(),
                filters.favorite,
            )?
        } else {
            self.session.tracks.iter().map(|track| track.id).collect()
        };

        if query
            .residual_term
            .as_deref()
            .is_none_or(|term| term.trim().is_empty())
        {
            return Ok(filtered_ids);
        }

        let residual = query.residual_term.as_deref().unwrap_or_default();
        if !filters.has_any() {
            let matcher = SkimMatcherV2::default();
            let ranked =
                crate::features::search::search_tracks(&matcher, &self.session.tracks, residual);
            return Ok(ranked.iter().map(|track| track.id).collect());
        }

        let ranked_ids =
            rank_filtered_candidates_by_fuzzy(&self.session.tracks, &filtered_ids, residual);
        Ok(ranked_ids)
    }
}

fn parse_search_orchestration_query(raw_query: &str) -> SearchOrchestrationQuery {
    let mut filters = StructuredSearchFilter::default();
    let mut residual_parts = Vec::new();

    for part in raw_query.split_whitespace() {
        let Some((key, value)) = part.split_once(':') else {
            residual_parts.push(part.to_string());
            continue;
        };

        if value.trim().is_empty() {
            residual_parts.push(part.to_string());
            continue;
        }

        match key.to_ascii_lowercase().as_str() {
            "artist" => filters.artist = Some(value.trim().to_string()),
            "album" => filters.album = Some(value.trim().to_string()),
            "fav" | "favorite" => {
                if let Some(parsed) = parse_favorite_value(value) {
                    filters.favorite = Some(parsed);
                } else {
                    residual_parts.push(part.to_string());
                }
            }
            _ => residual_parts.push(part.to_string()),
        }
    }

    let residual = residual_parts.join(" ").trim().to_string();
    SearchOrchestrationQuery {
        raw_query: raw_query.to_string(),
        structured_filters: filters,
        residual_term: if residual.is_empty() {
            None
        } else {
            Some(residual)
        },
    }
}

fn parse_favorite_value(raw: &str) -> Option<bool> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "y" | "on" => Some(true),
        "0" | "false" | "no" | "n" | "off" => Some(false),
        _ => None,
    }
}

fn rank_filtered_candidates_by_fuzzy(
    tracks: &[Track],
    candidate_ids: &[i64],
    keyword: &str,
) -> Vec<i64> {
    if keyword.trim().is_empty() {
        return candidate_ids.to_vec();
    }

    let matcher = SkimMatcherV2::default();
    let index: HashMap<i64, &Track> = tracks.iter().map(|track| (track.id, track)).collect();
    let mut scored = Vec::new();

    for track_id in candidate_ids {
        let Some(track) = index.get(track_id).copied() else {
            continue;
        };

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

        if let Some(score) = score {
            scored.push((*track_id, score));
        }
    }

    scored.sort_by(|(id_a, score_a), (id_b, score_b)| score_b.cmp(score_a).then(id_a.cmp(id_b)));
    scored.into_iter().map(|(track_id, _)| track_id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn track(id: i64, title: &str, artist: Option<&str>, album: Option<&str>) -> Track {
        Track {
            id,
            path: PathBuf::from(format!("/tmp/{id}.mp3")),
            mtime: 1,
            title: title.to_string(),
            artist: artist.map(ToString::to_string),
            album: album.map(ToString::to_string),
            duration_secs: Some(180),
            favorite: false,
            play_count: 0,
        }
    }

    #[test]
    fn parse_query_extracts_structured_filters_and_residual() {
        let parsed =
            parse_search_orchestration_query("artist:radiohead album:kid fav:yes idioteque");
        assert_eq!(
            parsed.structured_filters.artist.as_deref(),
            Some("radiohead")
        );
        assert_eq!(parsed.structured_filters.album.as_deref(), Some("kid"));
        assert_eq!(parsed.structured_filters.favorite, Some(true));
        assert_eq!(parsed.residual_term.as_deref(), Some("idioteque"));
    }

    #[test]
    fn parse_query_degrades_invalid_favorite_token_to_residual() {
        let parsed = parse_search_orchestration_query("fav:maybe bloom");
        assert_eq!(parsed.structured_filters.favorite, None);
        assert_eq!(parsed.residual_term.as_deref(), Some("fav:maybe bloom"));
    }

    #[test]
    fn fuzzy_ranking_stays_within_filtered_subset() {
        let tracks = vec![
            track(
                1,
                "Everything In Its Right Place",
                Some("Radiohead"),
                Some("Kid A"),
            ),
            track(2, "Angel", Some("Massive Attack"), Some("Mezzanine")),
            track(3, "Idioteque", Some("Radiohead"), Some("Kid A")),
        ];
        let filtered = vec![1, 3];
        let ranked = rank_filtered_candidates_by_fuzzy(&tracks, &filtered, "idio");
        assert_eq!(ranked, vec![3]);
    }
}
