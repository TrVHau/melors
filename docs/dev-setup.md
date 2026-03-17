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
├── app/          — app state + action modules
├── core/         — config modules + shared models
├── features/     — player, queue, search logic
├── services/     — scanner, metadata, storage modules
└── ui/           — input, render, and state submodules
```

See [architecture.md](architecture.md) for the full module breakdown.

## Key Dependencies

| Crate            | Purpose                         |
| ---------------- | ------------------------------- |
| `ratatui`        | Terminal UI framework           |
| `crossterm`      | Cross-platform terminal control |
| `rodio`          | Audio playback                  |
| `id3`            | ID3 tag parsing                 |
| `rusqlite`       | SQLite (bundled)                |
| `fuzzy-matcher`  | Fuzzy string matching           |
| `walkdir`        | Recursive directory scan        |
| `dirs`           | XDG-compliant path resolution   |
| `serde` + `toml` | Config file serialization       |
| `chrono`         | Clock visualizer timestamps     |

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
| Cache    | `~/.cache/melors/`                |
| Music    | `~/Music/melors/`                 |

To reset state, delete the database file and relaunch.
To point at a test music folder, edit `music_dir` in the config.

## Adding a New Action

1. Add the method to the appropriate action module in `src/app/actions/` (takes `&mut self`, returns `Result<_>`).
2. Wire the key in the appropriate input module in `src/ui/input/` (`normal.rs`, `modes.rs`, or `dispatch.rs`).
3. Add any required persistence method in `src/services/storage/`.
4. If UI state needs updating, modify `src/ui/state/model.rs` and related state helpers.
5. Run `cargo clippy --all-targets --all-features -- -D warnings` to validate.
