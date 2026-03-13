# MVP Specification

This document captures the confirmed MVP scope.

## In scope (MVP)

- Local MP3 library scan
- ID3 metadata extraction
- Library list UI (track list)
- Playback controls: play, pause, next, previous
- Seeking support
- Basic queue support
- Search (`/keyword`) with fast fuzzy matching
- Basic TUI layout:
  - sidebar
  - main list/content panel
  - now playing section
  - progress bar
- Keyboard navigation
- SQLite index and state persistence

## Out of scope (MVP)

- Visualizer (planned post-MVP)
- Playlist management UI/flows
- File watcher auto-sync
- Theme customization
- Advanced search syntax (`artist:`, `album:`)
- Keymap customization
- M3U import/export
- Mini-player mode

## Playback behavior

- Backend preference: `rodio`
- Repeat modes: `off`, `repeat_one`, `repeat_all`
- Shuffle: keep original order so user can return
- Seek keys:
  - Left: -5s
  - Right: +5s
  - Shift+Left: -10s
  - Shift+Right: +10s

## Search behavior

- Input style: `/keyword`
- Unified result list with mixed entity types
- Sort order:
  1. type priority: track > artist > album
  2. fuzzy score inside each type

## Scan behavior

- First run (empty DB): full scan accepted
- Next runs: incremental scan
- Incremental strategy: track by `path + mtime (+ optional hash)`
- No realtime watcher in MVP
- Rescan trigger in MVP: dedicated hotkey (not command mode)

## Missing/deleted file handling

- If file no longer exists at path: remove from library
- If file is in queue: remove queue entry
- If file is in playlist: keep playlist, mark entry as missing/unavailable
- Renamed file matching is not required in MVP (treat as old missing + new file)

## Persisted playback state

Persist in SQLite across app restarts:

- current queue
- current playing track
- playback position
- shuffle state
- repeat state

## Metadata fallback

If no ID3 title is available, use filename stem as title.

Example:

- file: `Daft Punk - Digital Love.mp3`
- title fallback: `Daft Punk - Digital Love`

## Performance targets

- Startup: first interactive screen < 300ms
- UI responsiveness target: < 16ms frame budget
- Incremental scan throughput target: about 1000 tracks/second

## Reliability requirements

- DB corruption policy:
  - backup old DB file
  - rebuild index from music library
- DB schema migration support required from early stage

## Release decision (MVP)

- Distribution focus: `cargo install`
- No telemetry/anonymized usage collection
