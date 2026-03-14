# Architecture

## Source Layout

```
src/
├── main.rs              — entry point; sets up App, terminal, and event loop
├── app/
│   ├── mod.rs           — App struct; owns all subsystems
│   ├── state.rs         — AppSession (tracks, queue, playback_state and version counters)
│   └── actions.rs       — all public mutations: play, seek, rename, queue ops, persistence
├── core/
│   ├── mod.rs
│   ├── config.rs        — reads/writes ~/.config/melors/config.toml
│   └── model.rs         — Track, PlaybackState, RepeatMode value types
├── features/
│   ├── mod.rs
│   ├── player.rs        — rodio sink wrapper; FFT analysis; volume; seek
│   ├── queue.rs         — queue ordering logic (shuffle, repeat)
│   └── search.rs        — fuzzy search over tracks using fuzzy-matcher
├── services/
│   ├── mod.rs
│   ├── scanner.rs       — walkdir scan; mtime-based incremental updates
│   ├── metadata.rs      — ID3 tag extraction with filename-stem fallback
│   └── storage.rs       — SQLite access layer (tracks, queue, playback state)
└── ui/
    ├── mod.rs
    ├── state.rs         — UiState: focus, InputMode, row caches, visualizer state
    ├── input.rs         — key event dispatch; search / rename input handlers
    └── render.rs        — ratatui draw functions for each panel
```

## Data Flow

```
 startup
   └─ load Config
   └─ open SQLite (auto-migrate)
   └─ load PlaybackState, Queue from DB
   └─ full scan (first run) or incremental scan
   └─ build AppSession { tracks, queue, playback_state }
   └─ start rodio output stream

 event loop (crossterm poll)
   ├─ key event → UiState::handle_key → App::action
   ├─ timer tick → App::refresh_playback_position
   │              └─ poll FFT analysis results
   │              └─ advance queue on track end
   │              └─ debounced DB write (250 ms)
   └─ render → UiState::draw (ratatui frame)

 shutdown
   └─ force-flush playback state to DB
   └─ restore terminal
```

## Threading Model

The app runs on a **single thread**. There are no background threads or async runtime.

- FFT analysis is submitted to a lightweight `std::thread::spawn` per file and results
  are polled each frame via an `mpsc` channel.
- All other work (scanning, DB writes) happens synchronously on the event-loop thread
  in response to user actions or timer ticks.

## SQLite Schema

### `tracks`

| Column           | Type    | Notes                                     |
| ---------------- | ------- | ----------------------------------------- |
| `id`             | INTEGER | primary key                               |
| `path`           | TEXT    | unique; absolute path to file             |
| `mtime`          | INTEGER | Unix timestamp; used for incremental scan |
| `title`          | TEXT    |                                           |
| `artist`         | TEXT    | nullable                                  |
| `album`          | TEXT    | nullable                                  |
| `duration_secs`  | INTEGER | nullable                                  |
| `favorite`       | INTEGER | 0 / 1                                     |
| `play_count`     | INTEGER | default 0                                 |
| `last_played_at` | TEXT    | nullable ISO timestamp                    |

### `queue_state`

| Column     | Type    | Notes          |
| ---------- | ------- | -------------- |
| `position` | INTEGER | sort order     |
| `track_id` | INTEGER | FK → tracks.id |

### `playback_state`

| Column             | Type    | Notes           |
| ------------------ | ------- | --------------- |
| `current_track_id` | INTEGER | nullable FK     |
| `position_secs`    | INTEGER |                 |
| `shuffle_enabled`  | INTEGER | 0 / 1           |
| `repeat_mode`      | TEXT    | off / one / all |
| `updated_at`       | TEXT    | ISO timestamp   |

## Caching Strategy

`UiState` maintains two render caches (Library and Queue) keyed by version counters
(`tracks_version`, `queue_version`) that increment on every mutation. A cache hit
skips all row formatting and track iteration for that frame.

Playback state writes are debounced: dirty writes are held for 250 ms and skipped
entirely if the value is unchanged from the last persisted snapshot.

## Visualizer

Three modes are rendered natively in ratatui — no external processes:

| Mode    | Key     | Technique                                               |
| ------- | ------- | ------------------------------------------------------- |
| Cava    | `Alt+1` | Real FFT (rustfft) on PCM frames; exponential smoothing |
| Clock   | `Alt+2` | 5-row block-glyph time display with date header         |
| CMatrix | `Alt+3` | Deterministic character rain seeded from track ID       |

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
