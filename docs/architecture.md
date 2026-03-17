# Architecture

## Source Layout

```
src/
├── main.rs                 — terminal bootstrap + event loop
├── app/
│   ├── mod.rs              — App facade and composition root
│   ├── state.rs            — AppSession and in-memory state versions
│   └── actions/
│       ├── mod.rs
│       ├── boot.rs         — startup/bootstrap actions
│       ├── playback.rs     — play/pause/seek/volume/repeat/shuffle
│       ├── queue.rs        — queue add/remove/reorder operations
│       ├── library.rs      — scan + library sync actions
│       ├── rename.rs       — rename and metadata update actions
│       └── session.rs      — persistence/readback of session state
├── core/
│   ├── mod.rs
│   ├── model.rs            — Track, PlaybackState, RepeatMode
│   └── config/
│       ├── mod.rs
│       ├── load.rs         — config load/save logic
│       └── paths.rs        — XDG/app path resolution
├── features/
│   ├── mod.rs
│   ├── queue.rs            — queue traversal and play order rules
│   ├── search.rs           — fuzzy search
│   └── player/
│       ├── mod.rs
│       ├── control.rs      — rodio playback control
│       └── analysis.rs     — FFT analysis + visualizer buffers
├── services/
│   ├── mod.rs
│   ├── metadata.rs         — ID3 read/write helpers
│   ├── scanner/
│   │   ├── mod.rs
│   │   ├── io.rs           — directory walking and file IO helpers
│   │   └── validate.rs     — scan target and path validation
│   └── storage/
│       ├── mod.rs
│       ├── migrations.rs   — schema creation/migration
│       ├── tracks.rs       — track CRUD
│       ├── playback.rs     — playback state persistence
│       └── queue.rs        — queue state persistence
└── ui/
  ├── mod.rs
  ├── input/
  │   ├── dispatch.rs     — mode-aware key dispatch
  │   ├── normal.rs       — normal-mode key handlers
  │   ├── modes.rs        — search/rename/tag-edit mode handlers
  │   └── selection.rs    — selection/focus navigation helpers
  ├── render/
  │   ├── chrome.rs       — tabs, borders, status chrome
  │   ├── lists.rs        — library/queue tables
  │   ├── playback.rs     — now playing + progress
  │   ├── visualizer.rs   — visualizer widgets
  │   └── theme.rs        — palette definitions
  └── state/
    ├── model.rs        — UiState shape and fields
    ├── mode.rs         — InputMode transitions
    └── cache.rs        — row cache update/invalidation
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

The app runs on a mostly **single-threaded** event loop.

- FFT analysis is submitted to a lightweight `std::thread::spawn` per file and results
  are polled each frame via an `mpsc` channel.
- All other work (scanning, DB writes) happens synchronously on the event-loop thread
  in response to user actions or timer ticks.

## SQLite Schema

### `tracks`

| Column            | Type    | Notes                                     |
| ----------------- | ------- | ----------------------------------------- |
| `id`              | INTEGER | primary key                               |
| `path`            | TEXT    | unique; absolute path to file             |
| `mtime`           | INTEGER | Unix timestamp; used for incremental scan |
| `title`           | TEXT    |                                           |
| `artist`          | TEXT    | nullable                                  |
| `album`           | TEXT    | nullable                                  |
| `duration`        | INTEGER | nullable                                  |
| `title_override`  | INTEGER | 0 / 1                                     |
| `artist_override` | INTEGER | 0 / 1                                     |
| `album_override`  | INTEGER | 0 / 1                                     |
| `favorite`        | INTEGER | 0 / 1                                     |
| `play_count`      | INTEGER | default 0                                 |
| `last_played_at`  | TEXT    | nullable ISO timestamp                    |

### `queue_state`

| Column     | Type    | Notes          |
| ---------- | ------- | -------------- |
| `position` | INTEGER | sort order     |
| `track_id` | INTEGER | FK → tracks.id |

### `playback_state`

| Column             | Type    | Notes                           |
| ------------------ | ------- | ------------------------------- |
| `current_track_id` | INTEGER | nullable FK                     |
| `position_secs`    | INTEGER |                                 |
| `shuffle_enabled`  | INTEGER | 0 / 1                           |
| `repeat_mode`      | INTEGER | 0=Off, 1=RepeatOne, 2=RepeatAll |
| `updated_at`       | TEXT    | ISO timestamp                   |

## Caching Strategy

`UiState` maintains render caches for the Library view keyed by version counters
such as `tracks_version`. A cache hit skips row formatting and track iteration
for that frame.

Playback state writes are debounced: dirty writes are held for 250 ms and skipped
entirely if the value is unchanged from the last persisted snapshot.

## Visualizer

Three modes are rendered natively in ratatui — no external processes:

| Mode    | Key     | Technique                                          |
| ------- | ------- | -------------------------------------------------- |
| Cava    | `Alt+1` | Deterministic demo bars with cached lane smoothing |
| Clock   | `Alt+2` | 5-row block-glyph time display with date header    |
| CMatrix | `Alt+3` | Deterministic character rain seeded from track ID  |

- `queue`: queue operations and ordering modes
- `search`: fuzzy search index and query execution
- `ui`: ratatui rendering + input handling
- `storage`: SQLite persistence and migrations
- `config`: load/save app configuration
