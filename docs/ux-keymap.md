# UX And Keymap

## Layout (MVP)

Main areas:

- left sidebar (sections/navigation)
- main content list (library/search results)
- now playing area
- bottom progress bar

Behavior on small terminal:

- prioritize metadata + progress visibility
- collapse non-critical sections first

## Interaction principles

- all core actions must be keyboard-accessible
- no mouse required
- predictable navigation state
- low-latency feedback on keypress

## Confirmed keys

Navigation and app flow:

- `j` / `k`: move up/down list
- `h` / `l`: move between panels
- `Enter`: select/open
- `q`: back when in sub-panel, exit when at root

Playback:

- `Space`: play/pause
- `n` / `p`: next/previous track
- `Left`: seek -5s
- `Right`: seek +5s
- `Shift+Left`: seek -10s
- `Shift+Right`: seek +10s

Search and utility:

- `/`: open search input
- `f`: toggle favorite on selected track
- `r` (recommended): trigger manual rescan

## Search UI

- single search mode in MVP (`/keyword`)
- result list mixes track, artist, album
- sorting:
  1. track
  2. artist
  3. album
- fuzzy score used inside each group

## Resume UX expectation

On app reopen, restore:

- queue
- current track
- playback position
- shuffle/repeat modes

User should feel app continues from previous session with minimal friction.
