# Roadmap

## Shipped (v0.1)

- MP3 library scanning with incremental mtime-based updates
- ID3 metadata extraction with filename-stem fallback
- Library list with current-track marker and favorite indicator
- Playback: play / pause / next / prev / seek / volume
- Fuzzy search across title, artist, and album (real-time)
- Queue with persistence across sessions
- Favorites and play-count tracking
- Repeat modes: off / one / all
- Shuffle
- Session persistence: queue, position, mode all restored on relaunch
- Inline rename: rename track files from within the TUI
- Visualizer panel with three modes:
  - Spectrum bars (real FFT via rustfft)
  - Big block-glyph clock
  - CMatrix character rain
- Render caches with version-counter invalidation
- Debounced SQLite writes (250 ms threshold)

## Near Term (v0.2)

- **Playlist management** — create, edit, and play named playlists
- **Statistics view** — most-played tracks, recently played list
- **File watcher** — auto-reload library when files change on disk
- **ID3 tag write-back** — persist rename to the file's ID3 title tag

## Medium Term (v0.3+)

- **M3U import/export** — interop with other players
- **Advanced search syntax** — `artist:`, `album:`, `year:` filters
- **Theme system** — configurable colors
- **Optional keymap customization** — rebind any key in config

## Non-Goals

- Cloud/streaming service integration
- Telemetry or usage analytics
- GUI mode
