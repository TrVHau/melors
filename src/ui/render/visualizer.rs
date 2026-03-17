use super::*;

impl UiState {
    pub(super) fn draw_visualizer_panel(
        &mut self,
        f: &mut ratatui::Frame<'_>,
        area: Rect,
        app: &App,
    ) {
        match self.visualizer_mode {
            VisualizerMode::Off => self.draw_off_visualizer(f, area),
            VisualizerMode::Cava => self.draw_cava_visualizer(f, area, app),
            VisualizerMode::Clock => self.draw_clock_visualizer(f, area),
            VisualizerMode::CMatrix => self.draw_cmatrix_visualizer(f, area, app),
        }
    }

    pub(super) fn draw_off_visualizer(&self, f: &mut ratatui::Frame<'_>, area: Rect) {
        let lines = self.centered_creeper_lines(area);
        let panel_bg = self.theme_visualizer_panel_bg_color();
        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Visualizer [Off] ")
                    .style(Style::default().bg(panel_bg))
                    .border_style(Style::default().fg(self.theme_dim_color())),
            )
            .style(Style::default().bg(panel_bg));
        f.render_widget(paragraph, area);
    }

    pub(super) fn draw_cava_visualizer(
        &mut self,
        f: &mut ratatui::Frame<'_>,
        area: Rect,
        app: &App,
    ) {
        let _ = app.visualizer_levels(1);
        let inner = self.visualizer_inner(area);
        let width = inner.width.max(1) as usize;
        let height = inner.height.max(1) as usize;
        let bars = ((width + 1) / 3).clamp(1, 40);
        let tick_ms = self.visualizer_tick() as u128;
        if self.cava_cached_levels.len() != bars
            || tick_ms.saturating_sub(self.visualizer_last_update_ms) >= 33
        {
            // Keep tick in a small range to avoid f32 precision loss from epoch-sized values.
            let tick = ((tick_ms as u64 % 600_000) as f32) / 1000.0;
            let seed = (self.cached_track_seed(app) % 997) as f32 / 997.0;
            let fresh: Vec<(f32, f32)> = (0..bars)
                .map(|idx| {
                    let lane = idx as f32 / bars as f32;
                    let drift = tick * 2.8;
                    let lane_phase = lane * std::f32::consts::TAU * 2.4 + drift + seed * 4.0;
                    let sweep = lane_phase.sin();
                    let ripple = (lane_phase * 1.8 + tick * 4.7).cos();
                    let bounce = (lane_phase * 0.6 + tick * 9.5).sin();
                    let level = ((sweep * 0.50 + ripple * 0.30 + bounce * 0.35) * 0.5 + 0.5)
                        .clamp(0.08, 1.0);
                    let peak = (level + 0.12 + (lane * 0.15)).clamp(0.0, 1.0);
                    (level, peak.clamp(0.0, 1.0))
                })
                .collect();

            if self.cava_cached_levels.len() == bars {
                for (idx, new_value) in fresh.into_iter().enumerate() {
                    let old = self.cava_cached_levels[idx];
                    let level = old.0 * 0.25 + new_value.0 * 0.75;
                    let peak = new_value.1.max(old.1 * 0.88);
                    self.cava_cached_levels[idx] = (level, peak);
                }
            } else {
                self.cava_cached_levels = fresh;
            }
            self.visualizer_last_update_ms = tick_ms;
        }

        let active_height = height.saturating_sub(1).max(1);
        let panel_bg = self.theme_visualizer_panel_bg_color();

        let mut lines = Vec::with_capacity(height);
        for row in (0..active_height).rev() {
            let mut row_text = String::with_capacity(bars * 3);
            for (level, peak) in &self.cava_cached_levels {
                let bar_height =
                    ((level * active_height as f32).round() as usize).clamp(0, active_height);
                let peak_height =
                    ((peak * active_height as f32).round() as usize).clamp(0, active_height);

                if peak_height == row + 1 && peak_height > 0 {
                    row_text.push_str("▓▓ ");
                } else if bar_height > row {
                    row_text.push_str("██ ");
                } else {
                    row_text.push_str("   ");
                }
            }

            lines.push(Line::from(vec![Span::styled(
                row_text,
                Style::default().fg(self.theme_visualizer_primary_color()),
            )]));
        }

        let baseline = "-  ".repeat(bars);
        lines.push(Line::from(vec![Span::styled(
            baseline,
            Style::default().fg(self.theme_visualizer_secondary_color()),
        )]));

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Visualizer [Demo Bars] ")
                    .style(Style::default().bg(panel_bg))
                    .border_style(Style::default().fg(self.theme_dim_color())),
            )
            .style(Style::default().bg(panel_bg));
        f.render_widget(paragraph, area);
    }

    pub(super) fn draw_clock_visualizer(&self, f: &mut ratatui::Frame<'_>, area: Rect) {
        let now = Local::now();
        let inner = self.visualizer_inner(area);
        let available_rows = inner.height.max(1) as usize;
        let panel_bg = self.theme_visualizer_panel_bg_color();

        let mut clock_rows = self.big_clock_lines(&now.format("%H:%M:%S").to_string());
        if clock_rows.len() > available_rows {
            clock_rows = self.sample_rows(&clock_rows, available_rows);
        }

        let mut lines = Vec::new();
        let can_show_date = available_rows >= clock_rows.len() + 2;
        if can_show_date {
            lines.push(self.centered_line(
                area.width,
                &now.format("%A %Y-%m-%d").to_string(),
                self.theme_visualizer_clock_date_color(),
            ));
            lines.push(Line::default());
        }

        for line in clock_rows {
            lines.push(self.centered_line(
                area.width,
                &line,
                self.theme_visualizer_clock_time_color(),
            ));
        }

        while lines.len() < available_rows {
            lines.push(Line::default());
        }

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Visualizer [Clock] ")
                    .style(Style::default().bg(panel_bg))
                    .border_style(Style::default().fg(self.theme_dim_color())),
            )
            .style(Style::default().bg(panel_bg));
        f.render_widget(paragraph, area);
    }

    pub(super) fn draw_cmatrix_visualizer(
        &mut self,
        f: &mut ratatui::Frame<'_>,
        area: Rect,
        app: &App,
    ) {
        let inner = self.visualizer_inner(area);
        let width = inner.width.max(1) as usize;
        let height = inner.height.max(1) as usize;
        let tick = self.visualizer_tick() as usize;
        let position = app.playback_state().position_secs.max(0) as usize;
        let charset = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let seed = self.cached_track_seed(app) as usize;
        let panel_bg = self.theme_visualizer_panel_bg_color();

        let mut lines = Vec::with_capacity(height);
        for row in 0..height {
            let mut spans = Vec::with_capacity(width);
            for col in 0..width {
                let speed = 1 + ((seed + col * 13) % 4);
                let trail = 4 + ((seed / 7 + col * 3) % 8);
                let offset = (seed / 11 + col * 17 + position * 3) % (height + trail + 8);
                let head = (tick / speed + offset) % (height + trail + 8);

                if row <= head && head - row < trail {
                    let glyph_index =
                        (seed + tick + row * 19 + col * 23 + position) % charset.len();
                    let ch = charset[glyph_index] as char;
                    let distance = head - row;
                    let color = if distance == 0 {
                        Color::White
                    } else if distance <= 2 {
                        Color::Rgb(120, 255, 120)
                    } else {
                        Color::Rgb(0, 120, 0)
                    };
                    spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                } else if (seed + tick + row * 5 + col * 3) % 37 == 0 {
                    spans.push(Span::styled(".", Style::default().fg(Color::Rgb(0, 60, 0))));
                } else {
                    spans.push(Span::raw(" "));
                }
            }
            lines.push(Line::from(spans));
        }

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Visualizer [CMatrix] ")
                    .style(Style::default().bg(panel_bg))
                    .border_style(Style::default().fg(self.theme_dim_color())),
            )
            .style(Style::default().bg(panel_bg));
        f.render_widget(paragraph, area);
    }

    pub(super) fn visualizer_tick(&self) -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as f64
    }

    pub(super) fn visualizer_inner(&self, area: Rect) -> Rect {
        area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 1,
        })
    }

    pub(super) fn big_clock_lines(&self, text: &str) -> Vec<String> {
        const HEIGHT: usize = 5;
        let mut lines = vec![String::new(); HEIGHT];

        for ch in text.chars() {
            let glyph = match ch {
                '0' => ["███", "█ █", "█ █", "█ █", "███"],
                '1' => [" █ ", "██ ", " █ ", " █ ", "███"],
                '2' => ["███", "  █", "███", "█  ", "███"],
                '3' => ["███", "  █", "███", "  █", "███"],
                '4' => ["█ █", "█ █", "███", "  █", "  █"],
                '5' => ["███", "█  ", "███", "  █", "███"],
                '6' => ["███", "█  ", "███", "█ █", "███"],
                '7' => ["███", "  █", "  █", "  █", "  █"],
                '8' => ["███", "█ █", "███", "█ █", "███"],
                '9' => ["███", "█ █", "███", "  █", "███"],
                ':' => ["   ", " █ ", "   ", " █ ", "   "],
                _ => ["   ", "   ", "   ", "   ", "   "],
            };

            for (row, part) in glyph.iter().enumerate() {
                lines[row].push_str(part);
                lines[row].push(' ');
            }
        }

        lines
    }

    pub(super) fn sample_rows(&self, rows: &[String], target: usize) -> Vec<String> {
        if target == 0 {
            return Vec::new();
        }
        if rows.len() <= target {
            return rows.to_vec();
        }

        (0..target)
            .map(|idx| {
                let src = idx * rows.len() / target;
                rows[src].clone()
            })
            .collect()
    }

    pub(super) fn centered_line(&self, width: u16, text: &str, color: Color) -> Line<'static> {
        let available = width.saturating_sub(2) as usize;
        let text_width = text.chars().count();
        let left_pad = available.saturating_sub(text_width) / 2;
        let mut content = String::new();
        content.push_str(&" ".repeat(left_pad));
        content.push_str(text);
        Line::from(vec![Span::styled(content, Style::default().fg(color))])
    }

    fn centered_creeper_lines(&self, area: Rect) -> Vec<Line<'static>> {
        // 8x8 editable template:
        // 0 = transparent(panel bg), 1 = green, 2 = lime, 3 = black, 4 = white.
        // Bạn chỉ cần sửa ma trận TILE bên dưới để vẽ lại creeper theo ý muốn.
        const TILE: [[u8; 8]; 8] = [
            [1, 1, 2, 2, 1, 1, 1, 2],
            [1, 1, 2, 1, 2, 2, 1, 1],
            [2, 3, 3, 1, 1, 3, 3, 1],
            [1, 3, 3, 2, 1, 3, 3, 1],
            [2, 1, 1, 3, 3, 2, 2, 1],
            [1, 2, 3, 3, 3, 3, 1, 2],
            [2, 1, 3, 3, 3, 3, 2, 1],
            [1, 2, 3, 1, 1, 3, 2, 1],
        ];
        let inner_h = area.height.saturating_sub(2) as usize;
        let inner_w = area.width.saturating_sub(2) as usize;
        if inner_h == 0 || inner_w == 0 {
            return Vec::new();
        }

        let colors = [
            self.theme_panel_bg_color(),
            Color::Rgb(88, 149, 52),
            Color::Rgb(132, 209, 78),
            Color::Rgb(10, 12, 10),
            Color::Rgb(236, 236, 236),
        ];
        let tile_h = TILE.len();
        let tile_w = TILE[0].len() * 2; // double width keeps pixel-ish proportions
        let offset_y = inner_h.saturating_sub(tile_h) / 2;
        let offset_x = inner_w.saturating_sub(tile_w) / 2;

        let mut lines: Vec<Line<'static>> = Vec::with_capacity(inner_h);
        for y in 0..inner_h {
            let mut spans: Vec<Span<'static>> = Vec::new();
            let mut run_start = 0usize;
            let mut run_style = Style::default().bg(colors[0]);

            for x in 0..inner_w {
                let style = if y >= offset_y
                    && y < offset_y + tile_h
                    && x >= offset_x
                    && x < offset_x + tile_w
                {
                    let ly = y - offset_y;
                    let lx = (x - offset_x) / 2;
                    let color_idx = TILE[ly][lx] as usize;
                    Style::default().bg(colors[color_idx])
                } else {
                    Style::default().bg(self.theme_visualizer_panel_bg_color())
                };

                if x == 0 {
                    run_style = style;
                    continue;
                }
                if style != run_style {
                    spans.push(Span::styled(" ".repeat(x - run_start), run_style));
                    run_start = x;
                    run_style = style;
                }
            }
            spans.push(Span::styled(" ".repeat(inner_w - run_start), run_style));
            lines.push(Line::from(spans));
        }
        lines
    }
}
