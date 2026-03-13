# Project Overview

## What is melors?

`melors` is a fast, elegant, local-first terminal music cockpit for personal MP3 libraries.

It is built for terminal-centric users (especially Linux users) who prefer keyboard workflows and minimal UI.

## Problem statement

Most music players are either:

- GUI-heavy and mouse-oriented
- cloud/streaming dependent
- not optimized for terminal workflows

`melors` solves this by providing a keyboard-native TUI for local music playback.

## Target users

- Linux desktop users
- keyboard-first power users
- users with local MP3 collections
- privacy-focused users who do not want cloud accounts

## Core product principles

1. Local-first: all files stay on user machine.
2. Keyboard-first: full navigation without mouse.
3. Terminal-native: polished TUI, not plain CLI.
4. Privacy-first: no telemetry in MVP.

## Defaults and paths

- Music directory: `~/Music/melors`
- Config file: `~/.config/melors/config.toml`
- SQLite database: `~/.local/share/melors/db.sqlite`
- Cache: `~/.cache/melors/`

## Success criteria

- First screen interactive in under 300ms
- stable local MP3 playback
- smooth keyboard navigation
- predictable resume behavior between sessions
