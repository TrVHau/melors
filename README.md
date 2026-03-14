# melors

> A keyboard-driven terminal music player for local MP3 libraries, written in Rust.

```
┌─ Library [Normal] ───────────────────────────────┐ ┌─ Queue ─────────────────┐
│-> * #0042 Tame Impala - Let It Happen            │ │  #0042 Let It Happen    │
│   * #0089 Radiohead - Reckoner                   │ │  #0089 Reckoner         │
│     #0103 Mac DeMarco - Chamber of Reflection    │ │  #0103 Chamber of...    │
│     #0211 LCD Soundsystem - All My Friends       │ │                         │
├─ Now Playing ────────────────────────────────────┤ ├─ Visualizer [Cava] ─────┤
│ Track: Tame Impala - Let It Happen               │ │ ██   ██ ▓▓ ██           │
│ Album: Currents | Repeat: off | Shuffle: Off     │ │ ██ ██ ██ ██ ██ ██       │
│ Volume: 80%  Status: Playing                     │ │ ████████████████████    │
├─ Progress ───────────────────────────────────────┤ │ -  -  -  -  -  -  -    │
│ [████████████░░░░░░░░░░░░░░░ 142s / 467s       ] │ └─────────────────────────┘
└──────────────────────────────────────────────────┘
```

## Features

- **Local-first** — scans `~/Music/melors/` on startup; no cloud, no accounts
- **Full keyboard control** — every action has a key binding, no mouse required
- **Fuzzy search** — matches across title, artist, and album in real time
- **Session persistence** — queue, playback position, repeat mode, and shuffle resume across restarts
- **Favorites & play counts** — mark tracks and track how often you play them
- **Inline rename** — rename track files directly from the TUI
- **Visualizer panel** — three built-in modes: spectrum bars (Cava), big clock, and CMatrix rain
- **Volume control** — per-session volume adjustment
- **Zero config to start** — sane defaults, directories created automatically

## Requirements

- Rust stable (≥ 1.85)
- Linux: ALSA headers (`libasound2-dev` on Debian/Ubuntu)
- macOS: CoreAudio (no extra packages needed)

## Install

```bash
git clone https://github.com/TrVHau/melors
cd melors
cargo build --release
# binary is at target/release/melors
```

Or run directly:

```bash
cargo run --release
```

## Paths

All directories are created on first launch — nothing to configure upfront.

| Purpose      | Path                              |
| ------------ | --------------------------------- |
| Music folder | `~/Music/melors/`                 |
| Config       | `~/.config/melors/config.toml`    |
| Database     | `~/.local/share/melors/db.sqlite` |

Drop `.mp3` files into `~/Music/melors/` and start the app. Press `r` at any time to rescan.

## Configuration

`~/.config/melors/config.toml` is created with defaults on first run:

```toml
music_dir = "/home/you/Music/melors"
```

Change `music_dir` to any path containing your MP3 collection.

## Keybindings

### Navigation

| Key         | Action                             |
| ----------- | ---------------------------------- |
| `↑` / `↓`   | Move selection up / down           |
| `Tab`       | Focus next panel (Library → Queue) |
| `Shift+Tab` | Focus previous panel               |
| `Enter`     | Play selected track or queue item  |
| `q`         | Quit                               |

### Playback

| Key                   | Action           |
| --------------------- | ---------------- |
| `Space`               | Play / pause     |
| `n`                   | Next track       |
| `p`                   | Previous track   |
| `←` / `→`             | Seek −5s / +5s   |
| `Shift+←` / `Shift+→` | Seek −10s / +10s |
| `[` / `]`             | Volume down / up |

### Library & Queue

| Key | Action                              |
| --- | ----------------------------------- |
| `s` | Open search                         |
| `f` | Toggle favorite on selected track   |
| `a` | Add selected track to queue         |
| `x` | Remove selected item from queue     |
| `m` | Rename selected track               |
| `e` | Cycle repeat mode (off / one / all) |
| `u` | Toggle shuffle                      |
| `r` | Rescan library from disk            |

### Visualizer

| Key     | Mode          |
| ------- | ------------- |
| `Alt+1` | Spectrum bars |
| `Alt+2` | Big clock     |
| `Alt+3` | CMatrix rain  |

## Search

Press `s` to enter search mode. Results filter in real time as you type.

| Key         | Action                                   |
| ----------- | ---------------------------------------- |
| `↑` / `↓`   | Navigate results                         |
| `Enter`     | Play selected result and exit search     |
| `Backspace` | Delete last character                    |
| `Esc`       | Cancel search and return to full library |

Search scores across track title, artist, and album using fuzzy matching.

## Rename

Press `m` on any track in the Library to rename it. The progress bar area becomes
a rename input field, prefilled with the current title.

| Key         | Action                                                      |
| ----------- | ----------------------------------------------------------- |
| Any key     | Edit the name                                               |
| `Backspace` | Delete last character                                       |
| `Enter`     | Confirm — renames the file on disk and updates the database |
| `Esc`       | Cancel, no changes made                                     |

The file is renamed in-place (same directory, same extension, new stem), and the
library reloads automatically.

## License

MIT — see [LICENSE](LICENSE).
