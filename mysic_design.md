
# mysic

**mysic** is a fast, elegant, local-first terminal music cockpit for personal MP3 libraries, built in Rust with a beautiful TUI interface and an integrated CAVA-style visualizer.

mysic is designed for terminal-centric users, especially Linux environments such as Hyprland, where keyboard-driven workflows and minimal UI are preferred.

---

# Philosophy

mysic follows four core principles.

## 1. Local-First

All music lives on the user's machine.

mysic does not depend on:

- streaming services  
- cloud accounts  
- internet connectivity  

The music library is scanned directly from a local directory (for example `~/Music/mysic`).

This ensures:

- fast startup
- offline usability
- full user control over files

---

## 2. Keyboard-First

mysic is designed to be fully usable without a mouse.

Navigation should feel natural for terminal users.

Typical controls:

```
j / k        move up and down
h / l        navigate panels
Enter        select item
Space        play / pause
/            search
n / p        next / previous track
q            back or quit
?            help
```

---

## 3. Terminal-Native Beauty

mysic is not just a CLI tool.

It is a **TUI application** with a polished layout.

Core UI ideas:

- sidebar navigation
- central content panel
- now-playing section
- playback bar
- subtle color theme
- smooth updates

The goal is to make the terminal feel alive.

---

## 4. Immersive Playback

mysic includes an integrated audio visualizer inspired by **CAVA**.

The visualizer transforms audio signals into frequency bars using FFT.

```
audio samples → FFT → frequency bins → terminal bars
```

Example visual output:

```
▁▂▃▄▅▆▇██▇▆▅▄▃▂
```

---

# Core Features

## Library Management

mysic scans a local directory and builds a music library.

Features:

- recursive MP3 scanning
- metadata extraction (ID3 tags)
- artist view
- album view
- track listing
- search

---

## Playback

Playback functionality includes:

- play / pause
- next / previous
- seek
- shuffle
- repeat
- queue management

Playback must be fast and stable.

---

## Playlist Support

Users can create and manage playlists locally.

Features:

- create playlist
- add/remove tracks
- reorder tracks
- save playlists

Playlists are stored locally.

---

## Search

Search should be fast and keyboard-friendly.

Possible modes:

```
/track
/artist
/album
```

Search should support fuzzy matching.

---

# Visualizer

The visualizer is inspired by CAVA but implemented internally.

Pipeline:

```
MP3 decode
   ↓
audio samples
   ↓
FFT analysis
   ↓
frequency bins
   ↓
terminal bar rendering
```

Features:

- 32–64 frequency bars
- smoothing / decay
- optional peak indicators
- theme color integration

---

# Application Layout

Example interface structure:

```
mysic
┌───────────────┬──────────────────────────────┐
│ Library       │ Now Playing                  │
│ Artists       │ Daft Punk - Digital Love     │
│ Albums        │                              │
│ Playlists     │ ▂▄▆██▆▄▂                     │
│ Queue         │ ▃▅▇██▇▅▃                     │
└───────────────┴──────────────────────────────┘

[██████████------] 3:21 / 6:09
```

Main UI areas:

- sidebar navigation
- main library panel
- now playing panel
- bottom playback bar

---

# Directory Structure

Music files:

```
~/Music/mysic/
```

Configuration:

```
~/.config/mysic/config.toml
```

Local database:

```
~/.local/share/mysic/db.sqlite
```

Cache:

```
~/.cache/mysic/
```

---

# Architecture Overview

```
mysic
 ├── scanner
 ├── metadata
 ├── library
 ├── player
 ├── visualizer
 ├── ui
 ├── storage
 └── config
```

## scanner

Scans the music directory and detects MP3 files.

## metadata

Extracts metadata from ID3 tags.

## library

Maintains the indexed music library.

## player

Handles audio playback and queue management.

## visualizer

Processes audio samples and generates visual bars.

## ui

Handles the TUI rendering and input system.

## storage

Manages the SQLite database.

## config

Loads and saves user configuration.

---

# Example Workflow

1. User places MP3 files in:

```
~/Music/mysic/
```

2. User launches the app:

```
mysic
```

3. mysic scans the library and builds the index.

4. User navigates with keyboard.

5. Music plays and the visualizer animates.

---

# Design Goals

mysic aims to be:

- fast
- minimal
- keyboard-centric
- visually elegant
- fully local

It should feel like a **music cockpit inside the terminal**.
