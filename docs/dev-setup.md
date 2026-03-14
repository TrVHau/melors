# Development Setup

## Prerequisites

| Tool          | Version | Notes                                                       |
| ------------- | ------- | ----------------------------------------------------------- |
| Rust (stable) | ≥ 1.85  | `rustup update stable`                                      |
| ALSA headers  | any     | Linux only: `sudo apt install libasound2-dev`               |
| SQLite        | bundled | `rusqlite` bundles its own SQLite; no system install needed |

## Clone and Build

```bash
git clone https://github.com/TrVHau/melors
cd melors
cargo build
```

Run with a release build for best audio performance:

```bash
cargo run --release
```

## Project Structure

```
src/
├── main.rs
├── app/          — App struct, session state, all action methods
├── core/         — config loader, shared value types (Track, PlaybackState)
├── features/     — player (rodio + FFT), queue logic, fuzzy search
├── services/     — file scanner, ID3 metadata reader, SQLite storage layer
└── ui/           — ratatui rendering, input dispatch, UI state and caches
```

See [architecture.md](architecture.md) for the full module breakdown.

## Key Dependencies

| Crate            | Purpose                              |
| ---------------- | ------------------------------------ |
| `ratatui`        | Terminal UI framework                |
| `crossterm`      | Cross-platform terminal control      |
| `rodio`          | Audio playback                       |
| `rustfft`        | Real FFT for the spectrum visualizer |
| `id3`            | ID3 tag parsing                      |
| `rusqlite`       | SQLite (bundled)                     |
| `fuzzy-matcher`  | Fuzzy string matching                |
| `walkdir`        | Recursive directory scan             |
| `dirs`           | XDG-compliant path resolution        |
| `serde` + `toml` | Config file serialization            |
| `chrono`         | Clock visualizer timestamps          |

## Running Tests

```bash
cargo test
```

## Linting

The project builds clean with `clippy` warnings-as-errors:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Useful Development Paths

| Purpose  | Default path during development   |
| -------- | --------------------------------- |
| Config   | `~/.config/melors/config.toml`    |
| Database | `~/.local/share/melors/db.sqlite` |
| Music    | `~/Music/melors/`                 |

To reset state, delete the database file and relaunch.
To point at a test music folder, edit `music_dir` in the config.

## Adding a New Action

1. Add the method to `src/app/actions.rs` (takes `&mut self`, returns `Result<_>`).
2. Wire the key in `src/ui/input.rs` inside `handle_key` or an input handler.
3. Add any required storage method to `src/services/storage.rs`.
4. If UI state needs updating, add or update fields in `src/ui/state.rs`.
5. Run `cargo clippy --all-targets --all-features -- -D warnings` to validate.
