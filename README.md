# melors

A keyboard-driven terminal music player for local MP3 libraries, written in Rust.

```
 > * #0042 Tame Impala - Let It Happen
   * #0089 Radiohead - Reckoner
     #0103 Mac DeMarco - Chamber of Reflection
     #0211 LCD Soundsystem - All My Friends
```

## Features

- Scans a local MP3 folder automatically on startup
- Reads ID3 tags (title, artist, album) with filename fallback
- Fuzzy search across tracks, artists, and albums
- Full keyboard control — no mouse required
- Remembers playback position, queue, and repeat mode between sessions
- Favorites and play count tracking
- Zero configuration needed out of the box

## Requirements

- Rust (stable)
- A working audio output device (ALSA on Linux, CoreAudio on macOS)

## Install

```bash
git clone https://github.com/TrVHau/melors
cd melors
cargo run --release
```

On first launch the app creates all directories it needs, including the music folder:

| Purpose      | Path                              |
| ------------ | --------------------------------- |
| Music folder | `~/Music/melors/`                 |
| Config       | `~/.config/melors/config.toml`    |
| Database     | `~/.local/share/melors/db.sqlite` |
| Cache        | `~/.cache/melors/`                |

Drop your `.mp3` files into `~/Music/melors/` and launch the app. That's it.

## Change The Music Folder

Open `~/.config/melors/config.toml` and set `music_dir` to any path you want:

```toml
music_dir = "/home/you/Music"
```

The app rescans this folder every time it starts and whenever you press `r`.

## Keybindings

### Navigation

| Key         | Action                            |
| ----------- | --------------------------------- |
| `↓`         | Move selection down               |
| `↑`         | Move selection up                 |
| `Shift+Tab` | Move focus left                   |
| `Tab`       | Move focus right                  |
| `Enter`     | Play selected track or queue item |
| `q`         | Quit                              |

### Playback

| Key                   | Action                       |
| --------------------- | ---------------------------- |
| `Space`               | Play / pause                 |
| `n`                   | Next track                   |
| `p`                   | Previous track               |
| `←` / `→`             | Seek −5s / +5s               |
| `Shift+←` / `Shift+→` | Seek −10s / +10s             |
| `]`                   | Volume up                    |
| `[`                   | Volume down                  |
| `Alt+1`               | Switch to Cava visualizer    |
| `Alt+2`               | Switch to Clock visualizer   |
| `Alt+3`               | Switch to CMatrix visualizer |

### Library

| Key | Action                            |
| --- | --------------------------------- |
| `s` | Open search                       |
| `f` | Toggle favorite on selected track |
| `a` | Add selected track to queue       |
| `x` | Remove selected queue item        |
| `r` | Rescan library from disk          |
| `e` | Cycle repeat mode                 |
| `u` | Toggle shuffle                    |

## Search

Press `s` to enter search mode. Type a keyword — results update as you type.

- `↑` / `↓` — move through results
- `Enter` — play the selected result
- `Backspace` — delete last character
- `Esc` — exit search, clear query, and return to full library
- `Enter` also clears the query after playing a result

Search matches across track title, artist, and album using fuzzy scoring.

## Visualizer Area

The lower-right panel now supports three code-native modes:

- `Cava` via `Alt+1`
- `Clock` via `Alt+2`
- `CMatrix` via `Alt+3`

These modes are rendered directly in the TUI code, not embedded from external tools.

## License

MIT — see [LICENSE](LICENSE).
