# UX and Keymap

## Panel Layout

```
┌───────────────────────────────────────────┐ ┌─────────────────────┐
│ Library [Normal / Search / Rename]      │ │ Queue                 │
│ (track list, scrollable)                │ │ (playback queue)      │
├───────────────────────────────────────────┤ ├─────────────────────┤
│ Now Playing                             │ │ Visualizer            │
│ (track, album, volume, status)          │ │ [Cava / Clock / CMatrix] │
├───────────────────────────────────────────┤ │                       │
│ Progress (or search / rename input)     │ │                       │
└───────────────────────────────────────────┘ └─────────────────────┘
```

The progress bar row doubles as an input area: it shows the search query during
Search mode and the rename input during Rename mode.

## Input Modes

| Mode     | Trigger              | Description                                          |
| -------- | -------------------- | ---------------------------------------------------- |
| `Normal` | default / `Esc`      | Browse and control playback                          |
| `Search` | `s`                  | Type a query; library filters in real time           |
| `Rename` | `m` / `M` on a track | Edit selected track title/file (`m`) or artist (`M`) |

## Keybindings

### Global (all modes)

| Key     | Action                      |
| ------- | --------------------------- |
| `Alt+1` | Visualizer: Spectrum (Cava) |
| `Alt+2` | Visualizer: Clock           |
| `Alt+3` | Visualizer: CMatrix rain    |

### Normal mode

#### Navigation

| Key         | Action                           |
| ----------- | -------------------------------- |
| `↑` / `↓`   | Move selection up / down         |
| `Tab`       | Focus next panel                 |
| `Shift+Tab` | Focus previous panel             |
| `Enter`     | Play selected track / queue item |
| `q`         | Quit                             |

#### Playback

| Key                   | Action            |
| --------------------- | ----------------- |
| `Space`               | Play / pause      |
| `n`                   | Next track        |
| `p`                   | Previous track    |
| `←` / `→`             | Seek −5s / +5s    |
| `Shift+←` / `Shift+→` | Seek −10s / +10s  |
| `[` / `]`             | Volume down / up  |
| `e`                   | Cycle repeat mode |
| `u`                   | Toggle shuffle    |

#### Library actions

| Key | Action                                |
| --- | ------------------------------------- |
| `s` | Enter Search mode                     |
| `f` | Toggle favorite on selected track     |
| `a` | Add selected track to queue           |
| `x` | Remove selected item from queue       |
| `m` | Enter Rename mode on selected track   |
| `M` | Enter Rename mode for selected artist |
| `r` | Rescan library from disk              |

### Search mode

| Key         | Action                                    |
| ----------- | ----------------------------------------- |
| Any char    | Append to query; library filters live     |
| `Backspace` | Delete last character                     |
| `↑` / `↓`   | Navigate filtered results                 |
| `Enter`     | Play selected result; exit Search mode    |
| `Esc`       | Cancel; restore full library; clear query |

### Rename mode

| Key         | Action                                                        |
| ----------- | ------------------------------------------------------------- |
| Any char    | Append to rename input                                        |
| `Backspace` | Delete last character                                         |
| `Enter`     | Confirm: apply title/file rename (`m`) or artist rename (`M`) |
| `Esc`       | Cancel; no changes made                                       |

## Interaction Principles

- All actions available without a mouse.
- Navigation state is preserved when switching panels.
- Search clears on both confirmation (`Enter`) and cancellation (`Esc`).
- `m` rename operates on the file on disk and updates the database atomically;
  if `std::fs::rename` fails the database is not touched.
- `M` rename updates only the artist field in the database.
- Status line (bottom of Now Playing) reflects the last action for quick feedback.

## Session Resume

On every launch the following state is restored from the database:

- Current track and playback position
- Queue contents and order
- Shuffle on/off
- Repeat mode (off / one / all)
