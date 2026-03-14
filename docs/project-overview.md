# Project Overview

## What is melors?

`melors` is a fast, local-first terminal music player for personal MP3 libraries. It runs entirely
in the terminal, requires no cloud account, and is controlled fully from the keyboard.

It is built for Linux and macOS users who prefer keyboard-centric workflows and want a polished TUI
rather than a plain CLI.

## Problem

Most music players are either GUI-heavy and mouse-oriented, cloud/streaming-dependent, or too
minimal to be daily-driver quality. `melors` fills the gap: a proper TUI player that treats local
files and keyboard navigation as first-class concerns.

## Target Users

- Linux desktop users who live in the terminal
- Keyboard-first power users
- People with local MP3 collections who avoid streaming services
- Privacy-focused users who do not want telemetry or accounts

## Design Principles

1. **Local-first** — all data stays on the user's machine.
2. **Keyboard-first** — every action has a key binding; mouse never required.
3. **Terminal-native** — polished TUI with multiple panels and a live visualizer.
4. **Privacy-first** — no telemetry, no network calls, no accounts.
5. **Zero config to start** — sane defaults; required directories created automatically.

## Default Paths

| Purpose      | Path                              |
| ------------ | --------------------------------- |
| Music folder | `~/Music/melors/`                 |
| Config       | `~/.config/melors/config.toml`    |
| Database     | `~/.local/share/melors/db.sqlite` |

All paths are configurable via `config.toml`.

## Performance Targets

- First interactive frame: < 300 ms after launch
- UI frame budget: ≤ 16 ms (60 fps capable)
- Incremental scan: ≥ 1 000 tracks / second
