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
        }
    }
}
