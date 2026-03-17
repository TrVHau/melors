use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::time::{Duration, Instant, UNIX_EPOCH};

use anyhow::Result;
use walkdir::WalkDir;

use super::validate;

#[derive(Debug, Clone)]
pub struct WatcherEventBatch {
    pub generation: u64,
    pub dirty_paths: HashSet<String>,
    pub window_elapsed_ms: u128,
}

pub fn spawn_watcher_runtime(
    music_dir: PathBuf,
    debounce_window: Duration,
    poll_interval: Duration,
) -> Result<Receiver<std::result::Result<WatcherEventBatch, String>>> {
    let (tx, rx) = channel();
    thread::Builder::new()
        .name(String::from("melors-watcher"))
        .spawn(move || {
            let mut previous = snapshot_mp3_state(&music_dir).unwrap_or_default();
            let mut debounce = DebounceCoordinator::new(debounce_window);

            loop {
                thread::sleep(poll_interval);
                let now = Instant::now();

                let current = match snapshot_mp3_state(&music_dir) {
                    Ok(snapshot) => snapshot,
                    Err(err) => {
                        if tx.send(Err(err.to_string())).is_err() {
                            break;
                        }
                        continue;
                    }
                };

                let changed = detect_changed_paths(&previous, &current);
                if !changed.is_empty() {
                    debounce.ingest(changed, now);
                }

                if let Some(batch) = debounce.try_emit(now)
                    && tx.send(Ok(batch)).is_err()
                {
                    break;
                }

                previous = current;
            }
        })?;

    Ok(rx)
}

#[derive(Debug)]
struct DebounceCoordinator {
    debounce_window: Duration,
    dirty_paths: HashSet<String>,
    first_event_at: Option<Instant>,
    last_event_at: Option<Instant>,
    next_generation: u64,
}

impl DebounceCoordinator {
    fn new(debounce_window: Duration) -> Self {
        Self {
            debounce_window,
            dirty_paths: HashSet::new(),
            first_event_at: None,
            last_event_at: None,
            next_generation: 1,
        }
    }

    fn ingest(&mut self, dirty_paths: HashSet<String>, at: Instant) {
        if self.first_event_at.is_none() {
            self.first_event_at = Some(at);
        }
        self.last_event_at = Some(at);
        self.dirty_paths.extend(dirty_paths);
    }

    fn try_emit(&mut self, now: Instant) -> Option<WatcherEventBatch> {
        let last = self.last_event_at?;
        if now.duration_since(last) < self.debounce_window {
            return None;
        }

        if self.dirty_paths.is_empty() {
            self.first_event_at = None;
            self.last_event_at = None;
            return None;
        }

        let generation = self.next_generation;
        self.next_generation = self.next_generation.saturating_add(1);
        let window_elapsed_ms = self
            .first_event_at
            .map(|start| now.duration_since(start).as_millis())
            .unwrap_or(0);

        let dirty_paths = std::mem::take(&mut self.dirty_paths);
        self.first_event_at = None;
        self.last_event_at = None;
        Some(WatcherEventBatch {
            generation,
            dirty_paths,
            window_elapsed_ms,
        })
    }
}

fn snapshot_mp3_state(music_dir: &Path) -> Result<HashMap<String, i64>> {
    if !validate::music_dir_is_scannable(music_dir) {
        return Ok(HashMap::new());
    }

    let mut out = HashMap::new();
    for entry in WalkDir::new(music_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();
        if !entry.file_type().is_file() || !validate::is_mp3(path) {
            continue;
        }

        let canonical = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let mtime = fs::metadata(&canonical)?
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        out.insert(canonical.to_string_lossy().to_string(), mtime);
    }
    Ok(out)
}

fn detect_changed_paths(
    previous: &HashMap<String, i64>,
    current: &HashMap<String, i64>,
) -> HashSet<String> {
    let mut out = HashSet::new();

    for (path, mtime) in current {
        if previous.get(path) != Some(mtime) {
            out.insert(path.clone());
        }
    }

    for path in previous.keys() {
        if !current.contains_key(path) {
            out.insert(path.clone());
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debounce_coalesces_burst_into_single_batch() {
        let start = Instant::now();
        let mut d = DebounceCoordinator::new(Duration::from_millis(50));

        let mut first = HashSet::new();
        first.insert(String::from("/tmp/a.mp3"));
        d.ingest(first, start);

        let mut second = HashSet::new();
        second.insert(String::from("/tmp/b.mp3"));
        d.ingest(second, start + Duration::from_millis(10));

        assert!(d.try_emit(start + Duration::from_millis(40)).is_none());
        let batch = d
            .try_emit(start + Duration::from_millis(61))
            .expect("batch emitted");
        assert_eq!(batch.generation, 1);
        assert_eq!(batch.dirty_paths.len(), 2);
    }

    #[test]
    fn debounce_generation_is_monotonic() {
        let start = Instant::now();
        let mut d = DebounceCoordinator::new(Duration::from_millis(1));

        let mut p = HashSet::new();
        p.insert(String::from("/tmp/a.mp3"));
        d.ingest(p, start);
        let b1 = d.try_emit(start + Duration::from_millis(2)).expect("first");

        let mut p2 = HashSet::new();
        p2.insert(String::from("/tmp/c.mp3"));
        d.ingest(p2, start + Duration::from_millis(3));
        let b2 = d
            .try_emit(start + Duration::from_millis(5))
            .expect("second");

        assert_eq!(b1.generation, 1);
        assert_eq!(b2.generation, 2);
    }

    #[test]
    fn detect_changed_paths_captures_creates_updates_and_deletes() {
        let mut previous = HashMap::new();
        previous.insert(String::from("/tmp/a.mp3"), 1);
        previous.insert(String::from("/tmp/b.mp3"), 2);

        let mut current = HashMap::new();
        current.insert(String::from("/tmp/a.mp3"), 3);
        current.insert(String::from("/tmp/c.mp3"), 1);

        let changed = detect_changed_paths(&previous, &current);
        assert!(changed.contains("/tmp/a.mp3"), "updated path detected");
        assert!(changed.contains("/tmp/b.mp3"), "deleted path detected");
        assert!(changed.contains("/tmp/c.mp3"), "new path detected");
        assert_eq!(changed.len(), 3);
    }
}
