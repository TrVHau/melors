# melors

`melors` is a local-first, keyboard-first terminal music player for personal MP3 libraries.

This project focuses on fast startup, smooth TUI interactions, and privacy-first local playback.

## Features (current)

- MP3 library scan (full + incremental)
- ID3 metadata extraction with filename fallback
- SQLite-backed track index and playback state persistence
- Terminal UI with keyboard navigation
- Playback with `rodio` (play/pause, seek, next/prev)
- Auto-next support based on queue and repeat mode
- Fuzzy search over library entries

## Project Structure

```text
src/
	app/         # app orchestration, actions, runtime session state
	core/        # domain models and configuration
	services/    # scanner, metadata reader, storage (SQLite)
	features/    # feature logic (player, search, queue)
	ui/          # TUI runtime, rendering, input, ui state
```

## Requirements

- Rust stable (latest recommended)
- Cargo
- A working audio output device (for playback)

## Quick Start

```bash
git clone <your-fork-or-this-repo-url>
cd melors
cargo run
```

Default paths used by the app:

- Config: `~/.config/melors/config.toml`
- Data/DB: `~/.local/share/melors/db.sqlite`
- Cache: `~/.cache/melors/`
- Music directory default: `~/Music/melors`

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
