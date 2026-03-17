use super::*;

impl UiState {
    pub(super) fn theme_panel_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(34, 34, 52),
            UiTheme::Forest => Color::Rgb(28, 40, 28),
            UiTheme::Ocean => Color::Rgb(22, 38, 52),
            UiTheme::Rose => Color::Rgb(50, 30, 42),
            UiTheme::Cyber => Color::Rgb(16, 16, 30),
            UiTheme::Lava => Color::Rgb(46, 18, 12),
            UiTheme::Aurora => Color::Rgb(18, 34, 30),
            UiTheme::Candy => Color::Rgb(44, 24, 44),
            UiTheme::Prism => Color::Rgb(20, 16, 34),
            UiTheme::Scanline => Color::Rgb(10, 18, 14),
        }
    }

    pub(super) fn theme_panel_alt_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(46, 46, 68),
            UiTheme::Forest => Color::Rgb(38, 54, 38),
            UiTheme::Ocean => Color::Rgb(30, 50, 68),
            UiTheme::Rose => Color::Rgb(66, 40, 54),
            UiTheme::Cyber => Color::Rgb(28, 28, 46),
            UiTheme::Lava => Color::Rgb(66, 28, 18),
            UiTheme::Aurora => Color::Rgb(30, 52, 44),
            UiTheme::Candy => Color::Rgb(66, 36, 66),
            UiTheme::Prism => Color::Rgb(28, 24, 48),
            UiTheme::Scanline => Color::Rgb(18, 28, 22),
        }
    }

    pub(super) fn theme_dim_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(70, 70, 95),
            UiTheme::Forest => Color::Rgb(58, 84, 58),
            UiTheme::Ocean => Color::Rgb(58, 92, 112),
            UiTheme::Rose => Color::Rgb(96, 66, 82),
            UiTheme::Cyber => Color::Rgb(78, 88, 132),
            UiTheme::Lava => Color::Rgb(130, 78, 54),
            UiTheme::Aurora => Color::Rgb(72, 120, 106),
            UiTheme::Candy => Color::Rgb(126, 88, 132),
            UiTheme::Prism => Color::Rgb(82, 86, 146),
            UiTheme::Scanline => Color::Rgb(54, 96, 78),
        }
    }

    pub(super) fn theme_header_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(200, 180, 255),
            UiTheme::Forest => Color::Rgb(175, 235, 175),
            UiTheme::Ocean => Color::Rgb(165, 225, 255),
            UiTheme::Rose => Color::Rgb(255, 190, 220),
            UiTheme::Cyber => Color::Rgb(104, 240, 255),
            UiTheme::Lava => Color::Rgb(255, 164, 96),
            UiTheme::Aurora => Color::Rgb(150, 255, 214),
            UiTheme::Candy => Color::Rgb(255, 170, 240),
            UiTheme::Prism => Color::Rgb(166, 196, 255),
            UiTheme::Scanline => Color::Rgb(152, 255, 178),
        }
    }

    pub(super) fn theme_library_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Green,
            UiTheme::Forest => Color::Rgb(110, 220, 110),
            UiTheme::Ocean => Color::Rgb(90, 220, 255),
            UiTheme::Rose => Color::Rgb(255, 150, 195),
            UiTheme::Cyber => Color::Rgb(64, 255, 228),
            UiTheme::Lava => Color::Rgb(255, 112, 54),
            UiTheme::Aurora => Color::Rgb(92, 255, 170),
            UiTheme::Candy => Color::Rgb(255, 110, 225),
            UiTheme::Prism => Color::Rgb(255, 98, 178),
            UiTheme::Scanline => Color::Rgb(118, 255, 120),
        }
    }

    pub(super) fn theme_queue_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Yellow,
            UiTheme::Forest => Color::Rgb(190, 230, 160),
            UiTheme::Ocean => Color::Rgb(250, 235, 150),
            UiTheme::Rose => Color::Rgb(255, 220, 165),
            UiTheme::Cyber => Color::Rgb(255, 94, 210),
            UiTheme::Lava => Color::Rgb(255, 205, 80),
            UiTheme::Aurora => Color::Rgb(124, 220, 255),
            UiTheme::Candy => Color::Rgb(255, 210, 120),
            UiTheme::Prism => Color::Rgb(255, 212, 86),
            UiTheme::Scanline => Color::Rgb(255, 214, 84),
        }
    }

    pub(super) fn theme_now_playing_row_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(255, 210, 80),
            UiTheme::Forest => Color::Rgb(210, 255, 170),
            UiTheme::Ocean => Color::Rgb(255, 235, 170),
            UiTheme::Rose => Color::Rgb(255, 230, 185),
            UiTheme::Cyber => Color::Rgb(255, 150, 230),
            UiTheme::Lava => Color::Rgb(255, 220, 120),
            UiTheme::Aurora => Color::Rgb(188, 255, 206),
            UiTheme::Candy => Color::Rgb(255, 220, 180),
            UiTheme::Prism => Color::Rgb(140, 255, 222),
            UiTheme::Scanline => Color::Rgb(186, 255, 188),
        }
    }

    pub(super) fn theme_progress_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Magenta,
            UiTheme::Forest => Color::Rgb(95, 210, 130),
            UiTheme::Ocean => Color::Rgb(80, 190, 255),
            UiTheme::Rose => Color::Rgb(255, 120, 170),
            UiTheme::Cyber => Color::Rgb(118, 94, 255),
            UiTheme::Lava => Color::Rgb(255, 88, 48),
            UiTheme::Aurora => Color::Rgb(72, 225, 190),
            UiTheme::Candy => Color::Rgb(255, 118, 188),
            UiTheme::Prism => Color::Rgb(110, 148, 255),
            UiTheme::Scanline => Color::Rgb(84, 255, 210),
        }
    }

    pub(super) fn theme_status_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(190, 190, 215),
            UiTheme::Forest => Color::Rgb(190, 230, 190),
            UiTheme::Ocean => Color::Rgb(180, 220, 235),
            UiTheme::Rose => Color::Rgb(235, 200, 220),
            UiTheme::Cyber => Color::Rgb(196, 212, 255),
            UiTheme::Lava => Color::Rgb(245, 196, 170),
            UiTheme::Aurora => Color::Rgb(196, 240, 220),
            UiTheme::Candy => Color::Rgb(244, 206, 236),
            UiTheme::Prism => Color::Rgb(226, 228, 255),
            UiTheme::Scanline => Color::Rgb(180, 228, 190),
        }
    }

    pub(super) fn theme_library_panel_bg_color(&self, is_active: bool) -> Color {
        match (self.theme, is_active) {
            (UiTheme::Prism, true) => Color::Rgb(46, 20, 66),
            (UiTheme::Prism, false) => Color::Rgb(24, 14, 42),
            (UiTheme::Scanline, true) => Color::Rgb(20, 34, 22),
            (UiTheme::Scanline, false) => Color::Rgb(10, 20, 14),
            (_, true) => self.theme_panel_alt_bg_color(),
            (_, false) => self.theme_panel_bg_color(),
        }
    }

    pub(super) fn theme_playback_panel_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(20, 40, 68),
            UiTheme::Scanline => Color::Rgb(10, 26, 20),
            _ => self.theme_panel_bg_color(),
        }
    }

    pub(super) fn theme_progress_panel_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(28, 54, 36),
            UiTheme::Scanline => Color::Rgb(14, 32, 24),
            _ => self.theme_panel_bg_color(),
        }
    }

    pub(super) fn theme_visualizer_panel_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(62, 32, 22),
            UiTheme::Scanline => Color::Rgb(6, 18, 12),
            _ => self.theme_panel_bg_color(),
        }
    }

    pub(super) fn theme_modal_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(36, 26, 58),
            UiTheme::Scanline => Color::Rgb(14, 28, 18),
            _ => self.theme_panel_alt_bg_color(),
        }
    }

    pub(super) fn theme_modal_border_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(255, 126, 214),
            UiTheme::Scanline => Color::Rgb(118, 255, 120),
            _ => self.theme_library_color(),
        }
    }

    pub(super) fn theme_modal_highlight_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(120, 232, 255),
            UiTheme::Scanline => Color::Rgb(142, 255, 164),
            _ => self.theme_header_color(),
        }
    }

    pub(super) fn theme_modal_warning_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(255, 142, 96),
            UiTheme::Cyber => Color::Rgb(255, 126, 184),
            UiTheme::Candy => Color::Rgb(255, 138, 120),
            UiTheme::Scanline => Color::Rgb(255, 196, 84),
            _ => Color::Red,
        }
    }

    pub(super) fn theme_popup_active_color(&self) -> Color {
        match self.theme {
            UiTheme::Lava => Color::Rgb(255, 126, 74),
            UiTheme::Prism => Color::Rgb(132, 236, 255),
            UiTheme::Scanline => Color::Rgb(146, 255, 172),
            _ => self.theme_header_color(),
        }
    }

    pub(super) fn theme_popup_inactive_color(&self) -> Color {
        match self.theme {
            UiTheme::Cyber => Color::Rgb(144, 164, 220),
            UiTheme::Candy => Color::Rgb(220, 178, 214),
            UiTheme::Prism => Color::Rgb(214, 188, 255),
            UiTheme::Scanline => Color::Rgb(116, 178, 132),
            _ => self.theme_status_color(),
        }
    }

    pub(super) fn theme_popup_border_color(&self) -> Color {
        match self.theme {
            UiTheme::Lava => Color::Rgb(255, 110, 64),
            UiTheme::Cyber => Color::Rgb(90, 238, 255),
            UiTheme::Aurora => Color::Rgb(124, 255, 214),
            UiTheme::Prism => Color::Rgb(255, 126, 214),
            UiTheme::Scanline => Color::Rgb(102, 255, 156),
            _ => self.theme_header_color(),
        }
    }

    pub(super) fn theme_visualizer_primary_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(255, 146, 108),
            UiTheme::Scanline => Color::Rgb(128, 255, 150),
            _ => Color::Rgb(236, 236, 245),
        }
    }

    pub(super) fn theme_visualizer_secondary_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(255, 214, 116),
            UiTheme::Scanline => Color::Rgb(90, 176, 112),
            _ => Color::Rgb(150, 150, 170),
        }
    }

    pub(super) fn theme_visualizer_clock_date_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(140, 244, 255),
            UiTheme::Scanline => Color::Rgb(255, 228, 108),
            _ => Color::Rgb(245, 185, 175),
        }
    }

    pub(super) fn theme_visualizer_clock_time_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(255, 142, 216),
            UiTheme::Scanline => Color::Rgb(118, 255, 150),
            _ => Color::Rgb(244, 184, 180),
        }
    }

    pub(super) fn theme_visualizer_matrix_head_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::White,
            UiTheme::Forest => Color::Rgb(214, 255, 214),
            UiTheme::Ocean => Color::Rgb(214, 250, 255),
            UiTheme::Rose => Color::Rgb(255, 224, 236),
            UiTheme::Cyber => Color::Rgb(206, 244, 255),
            UiTheme::Lava => Color::Rgb(255, 232, 176),
            UiTheme::Aurora => Color::Rgb(224, 255, 236),
            UiTheme::Candy => Color::Rgb(255, 230, 246),
            UiTheme::Prism => Color::Rgb(255, 236, 190),
            UiTheme::Scanline => Color::Rgb(214, 255, 222),
        }
    }

    pub(super) fn theme_visualizer_matrix_trail_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(120, 255, 120),
            UiTheme::Forest => Color::Rgb(108, 224, 124),
            UiTheme::Ocean => Color::Rgb(96, 228, 255),
            UiTheme::Rose => Color::Rgb(255, 144, 198),
            UiTheme::Cyber => Color::Rgb(108, 255, 238),
            UiTheme::Lava => Color::Rgb(255, 138, 84),
            UiTheme::Aurora => Color::Rgb(104, 255, 188),
            UiTheme::Candy => Color::Rgb(255, 138, 226),
            UiTheme::Prism => Color::Rgb(120, 232, 255),
            UiTheme::Scanline => Color::Rgb(120, 255, 132),
        }
    }

    pub(super) fn theme_visualizer_matrix_fade_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(0, 120, 0),
            UiTheme::Forest => Color::Rgb(26, 114, 44),
            UiTheme::Ocean => Color::Rgb(22, 120, 144),
            UiTheme::Rose => Color::Rgb(126, 34, 76),
            UiTheme::Cyber => Color::Rgb(24, 118, 126),
            UiTheme::Lava => Color::Rgb(134, 46, 18),
            UiTheme::Aurora => Color::Rgb(20, 122, 94),
            UiTheme::Candy => Color::Rgb(126, 38, 122),
            UiTheme::Prism => Color::Rgb(126, 76, 38),
            UiTheme::Scanline => Color::Rgb(28, 110, 42),
        }
    }

    pub(super) fn theme_visualizer_matrix_dot_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(0, 60, 0),
            UiTheme::Forest => Color::Rgb(12, 66, 22),
            UiTheme::Ocean => Color::Rgb(12, 70, 86),
            UiTheme::Rose => Color::Rgb(72, 18, 42),
            UiTheme::Cyber => Color::Rgb(16, 66, 72),
            UiTheme::Lava => Color::Rgb(74, 24, 12),
            UiTheme::Aurora => Color::Rgb(10, 70, 52),
            UiTheme::Candy => Color::Rgb(76, 22, 72),
            UiTheme::Prism => Color::Rgb(82, 48, 20),
            UiTheme::Scanline => Color::Rgb(12, 62, 18),
        }
    }

    pub(super) fn theme_creeper_primary_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(88, 149, 52),
            UiTheme::Forest => Color::Rgb(72, 144, 70),
            UiTheme::Ocean => Color::Rgb(44, 126, 138),
            UiTheme::Rose => Color::Rgb(146, 74, 108),
            UiTheme::Cyber => Color::Rgb(38, 152, 146),
            UiTheme::Lava => Color::Rgb(170, 82, 36),
            UiTheme::Aurora => Color::Rgb(44, 150, 108),
            UiTheme::Candy => Color::Rgb(154, 76, 154),
            UiTheme::Prism => Color::Rgb(196, 102, 70),
            UiTheme::Scanline => Color::Rgb(64, 150, 82),
        }
    }

    pub(super) fn theme_creeper_secondary_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(132, 209, 78),
            UiTheme::Forest => Color::Rgb(120, 210, 118),
            UiTheme::Ocean => Color::Rgb(86, 198, 214),
            UiTheme::Rose => Color::Rgb(220, 124, 170),
            UiTheme::Cyber => Color::Rgb(76, 236, 222),
            UiTheme::Lava => Color::Rgb(255, 148, 84),
            UiTheme::Aurora => Color::Rgb(104, 236, 180),
            UiTheme::Candy => Color::Rgb(244, 134, 224),
            UiTheme::Prism => Color::Rgb(255, 184, 120),
            UiTheme::Scanline => Color::Rgb(126, 255, 146),
        }
    }

    pub(super) fn theme_creeper_shadow_color(&self) -> Color {
        match self.theme {
            UiTheme::Lava => Color::Rgb(20, 6, 6),
            UiTheme::Prism => Color::Rgb(18, 12, 22),
            _ => Color::Rgb(10, 12, 10),
        }
    }

    pub(super) fn theme_creeper_glow_color(&self) -> Color {
        match self.theme {
            UiTheme::Cyber => Color::Rgb(218, 250, 255),
            UiTheme::Prism => Color::Rgb(255, 242, 214),
            UiTheme::Scanline => Color::Rgb(230, 255, 234),
            _ => Color::Rgb(236, 236, 236),
        }
    }
}
