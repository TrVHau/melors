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
- `Tab`: cycle panel focus (`Library <-> Queue`)
- `Up` / `Down`: move selection
- `Enter`: play selected track/queue item

### Playback

- `Space`: play/pause
- `n`: next track
- `p`: previous track
- `Left` / `Right`: seek -5s / +5s
- `Shift+Left` / `Shift+Right`: seek -10s / +10s
- `[` / `]`: volume down/up

### Library and Queue

- `s`: search mode
- `r`: rescan music directory
- `f`: toggle favorite on selected track
- `a`: add selected track to queue
- `x`: remove selected queue item
- `Shift+Up` / `Shift+Down` (in Queue): reorder queue item

### Metadata Editing

- `m`: rename selected track title/file
- `M`: rename selected track artist
- `t`: open tag editor (title/artist/album)

Tag editor:

- `Tab`: next field
- `Enter`: save
- `Esc`: cancel

### Playback Modes and Visualizer

- `e`: cycle repeat (`Off -> One -> All`)
- `u`: toggle shuffle
- `Alt+1`: visualizer `Cava`
- `Alt+2`: visualizer `Clock`
- `Alt+3`: visualizer `CMatrix`
- `Alt+T`: cycle UI theme (`Neon -> Amber -> Mono -> Forest`)

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
		storage/
			mod.rs
			migrations.rs
			tracks.rs
			playback.rs
			queue.rs
		metadata.rs
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
```

## Development

```bash
cargo check
cargo run
```

## License

MIT. See `LICENSE`.
