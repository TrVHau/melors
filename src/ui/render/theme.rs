use super::*;

impl UiState {
    pub(super) fn theme_dim_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(70, 70, 95),
            UiTheme::Amber => Color::Rgb(92, 76, 54),
            UiTheme::Mono => Color::Rgb(95, 95, 95),
            UiTheme::Forest => Color::Rgb(58, 84, 58),
        }
    }

    pub(super) fn theme_muted_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(120, 120, 160),
            UiTheme::Amber => Color::Rgb(176, 145, 106),
            UiTheme::Mono => Color::Rgb(155, 155, 155),
            UiTheme::Forest => Color::Rgb(120, 155, 120),
        }
    }

    pub(super) fn theme_accent_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Cyan,
            UiTheme::Amber => Color::Rgb(255, 176, 90),
            UiTheme::Mono => Color::White,
            UiTheme::Forest => Color::Rgb(120, 220, 140),
        }
    }

    pub(super) fn theme_highlight_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::White,
            UiTheme::Amber => Color::Rgb(255, 236, 190),
            UiTheme::Mono => Color::White,
            UiTheme::Forest => Color::Rgb(214, 255, 214),
        }
    }

    pub(super) fn theme_header_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(200, 180, 255),
            UiTheme::Amber => Color::Rgb(255, 205, 130),
            UiTheme::Mono => Color::Rgb(225, 225, 225),
            UiTheme::Forest => Color::Rgb(175, 235, 175),
        }
    }

    pub(super) fn theme_library_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Green,
            UiTheme::Amber => Color::Rgb(255, 170, 60),
            UiTheme::Mono => Color::Rgb(210, 210, 210),
            UiTheme::Forest => Color::Rgb(110, 220, 110),
        }
    }

    pub(super) fn theme_queue_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Yellow,
            UiTheme::Amber => Color::Rgb(255, 215, 120),
            UiTheme::Mono => Color::Rgb(200, 200, 200),
            UiTheme::Forest => Color::Rgb(190, 230, 160),
        }
    }

    pub(super) fn theme_now_playing_row_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(255, 210, 80),
            UiTheme::Amber => Color::Rgb(255, 225, 140),
            UiTheme::Mono => Color::Rgb(240, 240, 240),
            UiTheme::Forest => Color::Rgb(210, 255, 170),
        }
    }

    pub(super) fn theme_progress_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Magenta,
            UiTheme::Amber => Color::Rgb(255, 140, 80),
            UiTheme::Mono => Color::Rgb(220, 220, 220),
            UiTheme::Forest => Color::Rgb(95, 210, 130),
        }
    }

    pub(super) fn theme_status_color(&self) -> Color {
        match self.theme {
            UiTheme::Neon => Color::Rgb(190, 190, 215),
            UiTheme::Amber => Color::Rgb(240, 210, 170),
            UiTheme::Mono => Color::Rgb(210, 210, 210),
            UiTheme::Forest => Color::Rgb(190, 230, 190),
        }
    }
}
