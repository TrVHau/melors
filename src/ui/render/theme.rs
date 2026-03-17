use super::*;

impl UiState {
    pub(super) fn theme_panel_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(34, 34, 52),
            UiTheme::Amber => Color::Rgb(48, 38, 28),
            UiTheme::Mono => Color::Rgb(42, 42, 42),
            UiTheme::Forest => Color::Rgb(28, 40, 28),
            UiTheme::Ocean => Color::Rgb(22, 38, 52),
            UiTheme::Rose => Color::Rgb(50, 30, 42),
            UiTheme::Desert => Color::Rgb(58, 42, 24),
            UiTheme::Ice => Color::Rgb(32, 44, 52),
            UiTheme::Cyber => Color::Rgb(16, 16, 30),
            UiTheme::Lava => Color::Rgb(46, 18, 12),
            UiTheme::Aurora => Color::Rgb(18, 34, 30),
            UiTheme::Candy => Color::Rgb(44, 24, 44),
            UiTheme::Prism => Color::Rgb(20, 16, 34),
        }
    }

    pub(super) fn theme_panel_alt_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(46, 46, 68),
            UiTheme::Amber => Color::Rgb(62, 49, 36),
            UiTheme::Mono => Color::Rgb(56, 56, 56),
            UiTheme::Forest => Color::Rgb(38, 54, 38),
            UiTheme::Ocean => Color::Rgb(30, 50, 68),
            UiTheme::Rose => Color::Rgb(66, 40, 54),
            UiTheme::Desert => Color::Rgb(72, 54, 34),
            UiTheme::Ice => Color::Rgb(44, 60, 70),
            UiTheme::Cyber => Color::Rgb(28, 28, 46),
            UiTheme::Lava => Color::Rgb(66, 28, 18),
            UiTheme::Aurora => Color::Rgb(30, 52, 44),
            UiTheme::Candy => Color::Rgb(66, 36, 66),
            UiTheme::Prism => Color::Rgb(28, 24, 48),
        }
    }

    pub(super) fn theme_dim_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(70, 70, 95),
            UiTheme::Amber => Color::Rgb(92, 76, 54),
            UiTheme::Mono => Color::Rgb(95, 95, 95),
            UiTheme::Forest => Color::Rgb(58, 84, 58),
            UiTheme::Ocean => Color::Rgb(58, 92, 112),
            UiTheme::Rose => Color::Rgb(96, 66, 82),
            UiTheme::Desert => Color::Rgb(110, 86, 62),
            UiTheme::Ice => Color::Rgb(82, 112, 126),
            UiTheme::Cyber => Color::Rgb(78, 88, 132),
            UiTheme::Lava => Color::Rgb(130, 78, 54),
            UiTheme::Aurora => Color::Rgb(72, 120, 106),
            UiTheme::Candy => Color::Rgb(126, 88, 132),
            UiTheme::Prism => Color::Rgb(82, 86, 146),
        }
    }

    pub(super) fn theme_header_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(200, 180, 255),
            UiTheme::Amber => Color::Rgb(255, 205, 130),
            UiTheme::Mono => Color::Rgb(225, 225, 225),
            UiTheme::Forest => Color::Rgb(175, 235, 175),
            UiTheme::Ocean => Color::Rgb(165, 225, 255),
            UiTheme::Rose => Color::Rgb(255, 190, 220),
            UiTheme::Desert => Color::Rgb(255, 222, 165),
            UiTheme::Ice => Color::Rgb(210, 242, 255),
            UiTheme::Cyber => Color::Rgb(104, 240, 255),
            UiTheme::Lava => Color::Rgb(255, 164, 96),
            UiTheme::Aurora => Color::Rgb(150, 255, 214),
            UiTheme::Candy => Color::Rgb(255, 170, 240),
            UiTheme::Prism => Color::Rgb(166, 196, 255),
        }
    }

    pub(super) fn theme_library_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Green,
            UiTheme::Amber => Color::Rgb(255, 170, 60),
            UiTheme::Mono => Color::Rgb(210, 210, 210),
            UiTheme::Forest => Color::Rgb(110, 220, 110),
            UiTheme::Ocean => Color::Rgb(90, 220, 255),
            UiTheme::Rose => Color::Rgb(255, 150, 195),
            UiTheme::Desert => Color::Rgb(255, 180, 95),
            UiTheme::Ice => Color::Rgb(160, 245, 255),
            UiTheme::Cyber => Color::Rgb(64, 255, 228),
            UiTheme::Lava => Color::Rgb(255, 112, 54),
            UiTheme::Aurora => Color::Rgb(92, 255, 170),
            UiTheme::Candy => Color::Rgb(255, 110, 225),
            UiTheme::Prism => Color::Rgb(255, 98, 178),
        }
    }

    pub(super) fn theme_queue_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Yellow,
            UiTheme::Amber => Color::Rgb(255, 215, 120),
            UiTheme::Mono => Color::Rgb(200, 200, 200),
            UiTheme::Forest => Color::Rgb(190, 230, 160),
            UiTheme::Ocean => Color::Rgb(250, 235, 150),
            UiTheme::Rose => Color::Rgb(255, 220, 165),
            UiTheme::Desert => Color::Rgb(245, 225, 140),
            UiTheme::Ice => Color::Rgb(240, 245, 170),
            UiTheme::Cyber => Color::Rgb(255, 94, 210),
            UiTheme::Lava => Color::Rgb(255, 205, 80),
            UiTheme::Aurora => Color::Rgb(124, 220, 255),
            UiTheme::Candy => Color::Rgb(255, 210, 120),
            UiTheme::Prism => Color::Rgb(255, 212, 86),
        }
    }

    pub(super) fn theme_now_playing_row_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(255, 210, 80),
            UiTheme::Amber => Color::Rgb(255, 225, 140),
            UiTheme::Mono => Color::Rgb(240, 240, 240),
            UiTheme::Forest => Color::Rgb(210, 255, 170),
            UiTheme::Ocean => Color::Rgb(255, 235, 170),
            UiTheme::Rose => Color::Rgb(255, 230, 185),
            UiTheme::Desert => Color::Rgb(255, 236, 170),
            UiTheme::Ice => Color::Rgb(255, 250, 190),
            UiTheme::Cyber => Color::Rgb(255, 150, 230),
            UiTheme::Lava => Color::Rgb(255, 220, 120),
            UiTheme::Aurora => Color::Rgb(188, 255, 206),
            UiTheme::Candy => Color::Rgb(255, 220, 180),
            UiTheme::Prism => Color::Rgb(140, 255, 222),
        }
    }

    pub(super) fn theme_progress_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Magenta,
            UiTheme::Amber => Color::Rgb(255, 140, 80),
            UiTheme::Mono => Color::Rgb(220, 220, 220),
            UiTheme::Forest => Color::Rgb(95, 210, 130),
            UiTheme::Ocean => Color::Rgb(80, 190, 255),
            UiTheme::Rose => Color::Rgb(255, 120, 170),
            UiTheme::Desert => Color::Rgb(245, 156, 85),
            UiTheme::Ice => Color::Rgb(125, 220, 255),
            UiTheme::Cyber => Color::Rgb(118, 94, 255),
            UiTheme::Lava => Color::Rgb(255, 88, 48),
            UiTheme::Aurora => Color::Rgb(72, 225, 190),
            UiTheme::Candy => Color::Rgb(255, 118, 188),
            UiTheme::Prism => Color::Rgb(110, 148, 255),
        }
    }

    pub(super) fn theme_status_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(190, 190, 215),
            UiTheme::Amber => Color::Rgb(240, 210, 170),
            UiTheme::Mono => Color::Rgb(210, 210, 210),
            UiTheme::Forest => Color::Rgb(190, 230, 190),
            UiTheme::Ocean => Color::Rgb(180, 220, 235),
            UiTheme::Rose => Color::Rgb(235, 200, 220),
            UiTheme::Desert => Color::Rgb(235, 205, 170),
            UiTheme::Ice => Color::Rgb(205, 232, 242),
            UiTheme::Cyber => Color::Rgb(196, 212, 255),
            UiTheme::Lava => Color::Rgb(245, 196, 170),
            UiTheme::Aurora => Color::Rgb(196, 240, 220),
            UiTheme::Candy => Color::Rgb(244, 206, 236),
            UiTheme::Prism => Color::Rgb(226, 228, 255),
        }
    }

    pub(super) fn theme_library_panel_bg_color(&self, is_active: bool) -> Color {
        match (self.theme, is_active) {
            (UiTheme::Prism, true) => Color::Rgb(46, 20, 66),
            (UiTheme::Prism, false) => Color::Rgb(24, 14, 42),
            (_, true) => self.theme_panel_alt_bg_color(),
            (_, false) => self.theme_panel_bg_color(),
        }
    }

    pub(super) fn theme_playback_panel_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(20, 40, 68),
            _ => self.theme_panel_bg_color(),
        }
    }

    pub(super) fn theme_progress_panel_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(28, 54, 36),
            _ => self.theme_panel_bg_color(),
        }
    }

    pub(super) fn theme_visualizer_panel_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(62, 32, 22),
            _ => self.theme_panel_bg_color(),
        }
    }

    pub(super) fn theme_modal_bg_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(36, 26, 58),
            _ => self.theme_panel_alt_bg_color(),
        }
    }

    pub(super) fn theme_modal_border_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(255, 126, 214),
            _ => self.theme_library_color(),
        }
    }

    pub(super) fn theme_modal_highlight_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(120, 232, 255),
            _ => self.theme_header_color(),
        }
    }

    pub(super) fn theme_modal_warning_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(255, 142, 96),
            UiTheme::Cyber => Color::Rgb(255, 126, 184),
            UiTheme::Candy => Color::Rgb(255, 138, 120),
            _ => Color::Red,
        }
    }

    pub(super) fn theme_popup_active_color(&self) -> Color {
        match self.theme {
            UiTheme::Mono => Color::Rgb(245, 245, 245),
            UiTheme::Desert => Color::Rgb(255, 196, 112),
            UiTheme::Lava => Color::Rgb(255, 126, 74),
            UiTheme::Prism => Color::Rgb(132, 236, 255),
            _ => self.theme_header_color(),
        }
    }

    pub(super) fn theme_popup_inactive_color(&self) -> Color {
        match self.theme {
            UiTheme::Mono => Color::Rgb(188, 188, 188),
            UiTheme::Cyber => Color::Rgb(144, 164, 220),
            UiTheme::Candy => Color::Rgb(220, 178, 214),
            UiTheme::Prism => Color::Rgb(214, 188, 255),
            _ => self.theme_status_color(),
        }
    }

    pub(super) fn theme_popup_border_color(&self) -> Color {
        match self.theme {
            UiTheme::Lava => Color::Rgb(255, 110, 64),
            UiTheme::Cyber => Color::Rgb(90, 238, 255),
            UiTheme::Aurora => Color::Rgb(124, 255, 214),
            UiTheme::Prism => Color::Rgb(255, 126, 214),
            _ => self.theme_header_color(),
        }
    }

    pub(super) fn theme_visualizer_primary_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(255, 146, 108),
            _ => Color::Rgb(236, 236, 245),
        }
    }

    pub(super) fn theme_visualizer_secondary_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(255, 214, 116),
            _ => Color::Rgb(150, 150, 170),
        }
    }

    pub(super) fn theme_visualizer_clock_date_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(140, 244, 255),
            _ => Color::Rgb(245, 185, 175),
        }
    }

    pub(super) fn theme_visualizer_clock_time_color(&self) -> Color {
        match self.theme {
            UiTheme::Prism => Color::Rgb(255, 142, 216),
            _ => Color::Rgb(244, 184, 180),
        }
    }
}
