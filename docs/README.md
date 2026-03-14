# melors — Documentation

This folder contains the technical and product documentation for `melors`.

## Reading Order

| File                                       | Contents                                         |
| ------------------------------------------ | ------------------------------------------------ |
| [project-overview.md](project-overview.md) | What melors is, who it is for, design principles |
| [architecture.md](architecture.md)         | Module map, data flow, storage schema            |
| [ux-keymap.md](ux-keymap.md)               | TUI layout, keybindings, interaction model       |
| [roadmap.md](roadmap.md)                   | What's shipped and what's planned next           |
| [dev-setup.md](dev-setup.md)               | Setting up a local dev environment               |

## Quick Facts

- **Binary**: `melors`
- **Language**: Rust (edition 2024)
- **TUI**: `ratatui` + `crossterm`
- **Audio**: `rodio`
- **Storage**: `rusqlite` (bundled SQLite)
- **Minimum Rust**: stable ≥ 1.85
