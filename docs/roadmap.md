# Roadmap

## Naming transition

Project identity is moving from `mysic` to `melors`.

Planned rename scope:

- crate name
- binary name
- config/data/cache directory names
- docs and in-app branding

## Phase 0 - Foundation (current)

- finalize product and technical spec
- set up module skeleton
- define DB schema + migration strategy
- baseline TUI frame and input loop

## Phase 1 - MVP

- MP3 scanner (full first run + incremental updates)
- metadata extraction + fallback rules
- library list rendering
- playback controls (play/pause/next/prev/seek)
- queue persistence and resume state
- `/keyword` fuzzy search
- favorites, play count, last played
- DB backup + rebuild path on corruption

Exit criteria:

- first interactive screen in under 300ms (target)
- stable playback across typical local libraries
- no blocking UI stutters during normal operations

## Phase 2 - Post-MVP (v1.1)

- built-in visualizer (default 48 bars, 30 FPS)
- playlist management UX
- statistics views (most played/recent)
- optional file watcher
- theme customization

## Phase 3 - UX expansion (v1.2+)

- M3U import/export
- advanced search syntax
- optional keymap customization
- smarter rename/move reconciliation

## Non-goals

- cloud account integration
- streaming service integration
- telemetry collection
