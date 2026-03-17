# melors

Keyboard-first terminal MP3 player written in Rust.

## Highlights

- Local-only MP3 library scanning
- Fast keyboard workflow (no mouse)
- Search + queue + now playing panels
- Persistent playback state and queue
- Theme presets and visualizer modes
- Tag editing (title/artist/album) from TUI

## Quick Start

```bash
git clone https://github.com/TrVHau/melors
cd melors
cargo run --release
```

## Install

After the crate is published on crates.io:

```bash
cargo install melors
melors
```

Install directly from GitHub:

```bash
cargo install --git https://github.com/TrVHau/melors melors
melors
```

Linux build prerequisite (for audio backend):

```bash
sudo apt-get install -y pkg-config libasound2-dev
```

If your distro is not Debian/Ubuntu (for example Defora), install equivalent ALSA development packages using your system package manager.

## Runtime Paths

Created automatically on first run:

- Config: `~/.config/melors/config.toml`
- Database: `~/.local/share/melors/db.sqlite`
- Cache: `~/.cache/melors/`
- Default music dir: `~/Music/melors/`

Drop `.mp3` files in your music dir and press `r` to rescan anytime.

## Configuration

`~/.config/melors/config.toml`

```toml
music_dir = "/home/you/Music/melors"
```

## Keybindings

### App and Navigation

- `q`: quit
- `Up` / `Down`: move selection
- `Enter`: play selected track

### Playback

- `Space`: play/pause
- `n`: next track
- `p`: previous track
- `Left` / `Right`: seek -5s / +5s
- `Shift+Left` / `Shift+Right`: seek -10s / +10s
- `[` / `]`: volume down/up

### Library

- `s`: search mode
- `r`: rescan music directory
- `f`: toggle favorite on selected track
- `a`: open playlist picker to add selected (or currently playing) track

### Metadata Editing

- `t`: edit selected track metadata (title/artist/album). Saving will also rename filename using `Artist - Title` (or `Title` when artist is empty).

Tag editor:

- `Tab`: next field
- `Enter`: save
- `Esc`: cancel

### Playlist Quick Flow

1. Press `l` to open playlist modal.
   Playlist appears inline in the right pane (replacing Queue), not as popup.
   On narrow terminals, playlist view auto-switches to stacked mode for better readability.
2. Use `Up/Down` to choose playlist or item.
3. Press `Enter` on a playlist to open it; press `Enter` on `+ New playlist` to create a new one.
4. Press `a` in Library to open playlist picker, then `Enter` on a playlist (or `+ New playlist`) to add the track.
5. Press `d` in items view to remove the selected track from that playlist.
6. Press `r` in playlists view to rename the selected playlist, then `Enter` to save.
7. Press `Esc` in items view to go back to playlists; press `Esc` again to close.
8. Press `Enter` in items view to play.
9. Once a playlist is opened for playback, it becomes the current `Up Next` list in the main UI.

### Playback Modes and Visualizer

- `Alt+0`: visualizer `Off` (low-power mode)
- `e`: cycle repeat (`Off -> One -> All`)
- `u`: toggle shuffle
- `Alt+1`: visualizer `Demo Bars` (lightweight example)
- `Alt+2`: visualizer `Clock`
- `Alt+3`: visualizer `CMatrix`
- `Alt+T`: cycle UI theme (`Neon -> Amber -> Mono -> Forest -> Ocean -> Rose -> Desert -> Ice`)

## Project Structure

```text
src/
	app/
		actions/
			boot.rs
			playback.rs
			queue.rs
			library.rs
			rename.rs
			session.rs
		mod.rs
		state.rs
	core/
		config/
			mod.rs
			load.rs
			paths.rs
		model.rs
	features/
		player/
			mod.rs
			control.rs
			analysis.rs
		search.rs
	services/
		scanner/
			mod.rs
			io.rs
			validate.rs
			watcher.rs
		storage/
			mod.rs
			migrations.rs
			tracks.rs
			playback.rs
			queue.rs
		metadata.rs
	quality/
		mod.rs
		thresholds.rs
		evaluator.rs
		diagnostics.rs
	release/
		mod.rs
	ui/
		input/
			dispatch.rs
			normal.rs
			modes.rs
			selection.rs
		render/
			chrome.rs
			lists.rs
			playback.rs
			visualizer.rs
			theme.rs
		state/
			model.rs
			mode.rs
			cache.rs
config/
	quality-thresholds.toml
scripts/
	quality-gate.sh
```

## Development

```bash
cargo check
cargo run
cargo package
cargo publish --dry-run
```

## Quality Gate

Run the same quality gate locally as CI:

```bash
scripts/quality-gate.sh
```

Threshold registry:

- `config/quality-thresholds.toml`

## Release Readiness Gate

Run deterministic release/documentation readiness checks:

```bash
scripts/release-gate.sh
```

Outputs:

- `artifacts/release/checksums-<candidate-id>.txt`
- `artifacts/release/release-summary-<candidate-id>.md`
- `artifacts/release/approval-events.log`

Release gate inputs:

- `config/release-policy.toml`
- `config/docs-sync-checklist.md`
- Release notes reference path (`RELEASE_NOTES_PATH`, defaults to `docs/README.md`)

## License

MIT. See `LICENSE`.
