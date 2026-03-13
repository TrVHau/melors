# Development Setup

## Current repository state

- crate name in `Cargo.toml` is still `mysic`
- `src/main.rs` is currently bootstrap code
- documentation reflects target direction for `melors`

## Suggested immediate tasks

1. Rename crate and binary to `melors`.
2. Add core dependencies.
3. Create module skeleton.
4. Implement boot path with config + DB + basic UI frame.

## Suggested dependencies (initial)

- TUI/input: `ratatui`, `crossterm`
- audio: `rodio`
- metadata: `id3`
- database: `rusqlite` (or `sqlx` with sqlite)
- filesystem paths: `directories` or `dirs`
- fuzzy search: `nucleo-matcher` or `skim`-style matcher crate
- async/runtime (optional): `tokio`
- logging: `tracing`, `tracing-subscriber`

## Proposed source layout

`src/`

- `main.rs`
- `app/mod.rs`
- `config/mod.rs`
- `storage/mod.rs`
- `scanner/mod.rs`
- `metadata/mod.rs`
- `library/mod.rs`
- `player/mod.rs`
- `queue/mod.rs`
- `search/mod.rs`
- `ui/mod.rs`

## MVP implementation order

1. Config + path resolution (`~/.config/melors`, `~/.local/share/melors`)
2. SQLite init + migrations
3. Scanner (first full + incremental later)
4. Metadata extraction and `tracks` table writes
5. Basic TUI layout and list rendering
6. Playback and queue controls
7. Search integration
8. Persistence and restore on startup

## Testing focus

Priority tests:

- scanner correctness
- metadata parser fallback
- queue ordering and repeat/shuffle behavior
- search ranking and type priority

## Benchmark guidance

Large-library benchmarking is not required in early stage, but keep hooks ready:

- measure scan throughput
- measure startup readiness latency
- detect UI frame drops under load
