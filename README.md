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

| Key       | Action              |
| --------- | ------------------- |
| `j` / `↓` | Move selection down |
| `k` / `↑` | Move selection up   |
| `h`       | Focus sidebar       |
| `l`       | Focus library       |
| `Enter`   | Play selected track |
| `q`       | Quit                |

### Playback

| Key                   | Action           |
| --------------------- | ---------------- |
| `Space`               | Play / pause     |
| `n`                   | Next track       |
| `p`                   | Previous track   |
| `←` / `→`             | Seek −5s / +5s   |
| `Shift+←` / `Shift+→` | Seek −10s / +10s |

### Library

| Key | Action                            |
| --- | --------------------------------- |
| `/` | Open search                       |
| `f` | Toggle favorite on selected track |
| `r` | Rescan library from disk          |

## Search

Press `/` to enter search mode. Type a keyword — results update as you type.

- `j` / `k` — move through results
- `Enter` — play the selected result
- `Backspace` — delete last character
- `Esc` or `q` — exit search, go back to full library

Search matches across track title, artist, and album using fuzzy scoring.

## License

MIT — see [LICENSE](LICENSE).
