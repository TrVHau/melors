# melors

`melors` is a local-first, keyboard-first terminal music player for personal MP3 libraries.

It scans a local MP3 folder, stores metadata in SQLite, and lets you browse and control playback entirely from the keyboard.

## What It Does

- Scans a local MP3 library on startup
- Extracts ID3 metadata with filename fallback
- Persists library index and playback state in SQLite
- Plays audio in the terminal with keyboard controls
- Supports fuzzy search over tracks, artists, and albums
- Remembers the last playback position between sessions

## Requirements

- Rust stable
- Cargo
- A working audio output device
- A folder containing `.mp3` files

## Install And Run

```bash
git clone <your-fork-or-this-repo-url>
cd melors
cargo run
```

On first launch, `melors` creates its config and data directories automatically.

Default locations:

- Config: `~/.config/melors/config.toml`
- Database: `~/.local/share/melors/db.sqlite`
- Cache: `~/.cache/melors/`
- Default music folder: `~/Music/melors`

## Configure Your Music Folder

Edit `~/.config/melors/config.toml`:

```bash
music_dir = "/absolute/path/to/your/mp3-library"
```

Use a directory that contains `.mp3` files. The app rescans this folder on startup and when you trigger a manual rescan.

## How To Use

When the app starts, it scans your library and opens the terminal UI.

- `j` / `k`: move selection down or up
- `h` / `l`: switch focus between panels
- `Enter`: play the selected track
- `Space`: play or pause
- `n` / `p`: next or previous track
- `Left` / `Right`: seek backward or forward 5 seconds
- `Shift+Left` / `Shift+Right`: seek backward or forward 10 seconds
- `/`: open search mode
- `f`: toggle favorite on the selected track
- `r`: rescan the library from disk
- `q`: quit the app

## Search

Press `/`, type a keyword, then use `j` / `k` to move through the results. Press `Enter` to play the selected result, or `Esc` to leave search mode.

## Notes

- If the configured music folder does not exist yet, the app starts with an empty library.
- The current playback position is saved when playback state changes and again on exit.
- If files are removed from disk and you rescan, stale queue and playback references are cleaned up automatically.

## Development

```bash
# Format
cargo fmt

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Build check
cargo check
```

## Contributing

Contributions, bug reports, and feature proposals are welcome.

1. Fork this repository.
2. Create a feature branch from `main`.
3. Make focused changes with clear commit messages.
4. Run local checks (`fmt`, `clippy`, `check`).
5. Open a Pull Request.

Read full guidelines in `CONTRIBUTING.md`.

## License

This project is licensed under the MIT License. See `LICENSE`.
