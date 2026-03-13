# Architecture

## Module map

Planned logical modules:

- `scanner`: discovers MP3 files in music directory
- `metadata`: reads ID3 tags and fallback metadata
- `library`: in-memory library model + indexing state
- `player`: audio playback and transport controls
- `queue`: queue operations and ordering modes
- `search`: fuzzy search index and query execution
- `ui`: ratatui rendering + input handling
- `storage`: SQLite persistence and migrations
- `config`: load/save app configuration

## Data flow

1. App boot
2. Load config and open DB
3. Run migrations
4. Restore persisted playback/session state
5. Start scan task
   - first run: full scan
   - later runs: incremental scan
6. Build/update library model
7. Render TUI and accept input
8. Route actions to player/queue/search/storage

## Runtime model

Recommended runtime split:

- UI thread: input + rendering
- worker tasks:
  - scanner updates
  - metadata extraction
  - DB writes
  - playback event updates

Use message passing between UI and workers to keep UI responsive.

## Storage model (SQLite)

Suggested tables (initial shape):

- `tracks`
  - id
  - path (unique)
  - mtime
  - hash (optional)
  - title
  - artist
  - album
  - duration
  - favorite (bool)
  - play_count
  - last_played_at
- `queue_state`
  - id
  - position
  - track_id
- `playback_state`
  - current_track_id
  - position_secs
  - shuffle_enabled
  - repeat_mode
  - updated_at
- `playlists`
  - id
  - name
  - created_at
- `playlist_items`
  - playlist_id
  - track_id (nullable if missing)
  - original_path
  - is_missing
  - order_index
- `schema_migrations`
  - version
  - applied_at

## Scan reconciliation rules

During incremental scan:

- track exists in FS + changed mtime: re-read metadata and update record
- track exists in DB but missing in FS: mark/remove according to feature rule
- new path in FS not in DB: insert new track

## UI stack

- TUI: `ratatui`
- Terminal backend: `crossterm`
- Audio backend target: `rodio`

## Future extension points

- visualizer pipeline (post-MVP)
- playlist UX improvements
- watcher-based live update
- theme system
