# UX Keymap (Single Source of Truth)

This file documents the keymap currently implemented in `src/ui/input/*`.

## Modes

- `Normal`: default browsing and playback control.
- `Search`: query input and filtered library navigation.
- `EditTag`: edit title/artist/album for the selected track.
- `PlaylistModal`: manage playlists and playlist items.

## Global (`all modes`)

- `Alt+0`: visualizer off (low-power)
- `Alt+1`: visualizer demo bars
- `Alt+2`: visualizer clock
- `Alt+3`: visualizer cmatrix
- `Alt+T`: cycle theme (`Neon -> Amber -> Mono -> Forest -> Ocean -> Rose -> Desert -> Ice`)

## Normal Mode

### Navigation

- `q`: quit
- `Up` / `Down`: selection up/down
- `Enter`: play selected track

### Playback

- `Space`: toggle play/pause
- `n`: next track
- `p`: previous track
- `Left` / `Right`: seek -5s / +5s
- `Shift+Left` / `Shift+Right`: seek -10s / +10s
- `[ / ]`: volume down/up
- `e`: cycle repeat mode
- `u`: toggle shuffle

### Library Actions

- `s`: enter search mode
- `l`: open playlist modal
- `r`: rescan library
- `f`: toggle favorite
- `a`: quick add selected/current track into playlist
- `t` or `T`: enter tag editor

## Search Mode

- `Esc`: back to normal mode
- `Enter`: play selected search result and exit search mode
- `Backspace`: remove last query character
- `Up` / `Down`: move in filtered results
- any printable char: append to query

## EditTag Mode

- `Esc`: cancel
- `Tab` / `BackTab`: next field (Title -> Artist -> Album)
- `Enter`: save
- `Backspace`: delete character
- any printable char: append to active field

## Playlist Modal

Only four controls are used inside playlist mode:

- `Up` / `Down`: move selection
- `Enter`: open selected playlist, play selected item, or create a new playlist from the `+ New playlist` row
- `a` (from Library/Normal mode): opens playlist picker to choose target playlist for the selected/current track
- `d`: remove selected item from the opened playlist
- `r`: rename selected playlist (from playlist list)
- `Esc`: go back from items to playlists, or close the playlist modal from playlist list

## Notes

- The old rename mode (`m`, `M`) is no longer active; metadata editing is done via `t`/`T` only.
- Playlist naming defaults to automatic (`Playlist 1`, `Playlist 2`, ...) and can be changed with `r`.
- In playlist items view, modification is remove-only (`d`); adding is initiated from Library with `a`.
- Opening or playing a playlist makes that playlist become the current `Up Next` source.
- Status and header text are intentionally minimal; the UI no longer repeats full key hints on every screen.
