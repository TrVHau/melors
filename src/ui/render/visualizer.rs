use super::*;

impl UiState {
    pub(super) fn draw_visualizer_panel(
        &mut self,
        f: &mut ratatui::Frame<'_>,
        area: Rect,
        app: &App,
    ) {
        match self.visualizer_mode {
            VisualizerMode::Cava => self.draw_cava_visualizer(f, area, app),
            VisualizerMode::Clock => self.draw_clock_visualizer(f, area),
            VisualizerMode::CMatrix => self.draw_cmatrix_visualizer(f, area, app),
        }
    }

    pub(super) fn draw_cava_visualizer(
        &mut self,
        f: &mut ratatui::Frame<'_>,
        area: Rect,
        app: &App,
    ) {
        let inner = self.visualizer_inner(area);
        let width = inner.width.max(1) as usize;
        let height = inner.height.max(1) as usize;
        let bars = ((width + 1) / 3).clamp(1, 40);
        let tick_ms = self.visualizer_tick() as u128;
        if self.cava_cached_levels.len() != bars
            || tick_ms.saturating_sub(self.visualizer_last_update_ms) >= 33
        {
            let fresh = app.visualizer_levels(bars);
            if self.cava_cached_levels.len() == fresh.len() {
                for (idx, new_value) in fresh.iter().enumerate() {
                    let old = self.cava_cached_levels[idx];
                    let level = old.0 * 0.65 + new_value.0 * 0.35;
                    let peak = new_value.1.max(old.1 * 0.92);
                    self.cava_cached_levels[idx] = (level, peak);
                }
            } else {
                self.cava_cached_levels = fresh;
            }
            self.visualizer_last_update_ms = tick_ms;
        }

        let active_height = height.saturating_sub(1).max(1);

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
                Style::default().fg(Color::Rgb(236, 236, 245)),
            )]));
        }

        let baseline = "-  ".repeat(bars);
        lines.push(Line::from(vec![Span::styled(
            baseline,
            Style::default().fg(Color::Rgb(150, 150, 170)),
        )]));

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Visualizer [Cava] ")
                .border_style(Style::default().fg(Color::Rgb(178, 178, 210))),
        );
        f.render_widget(paragraph, area);
    }

    pub(super) fn draw_clock_visualizer(&self, f: &mut ratatui::Frame<'_>, area: Rect) {
        let now = Local::now();
        let inner = self.visualizer_inner(area);
        let available_rows = inner.height.max(1) as usize;

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
                Color::Rgb(245, 185, 175),
            ));
            lines.push(Line::default());
        }

        for line in clock_rows {
            lines.push(self.centered_line(area.width, &line, Color::Rgb(244, 184, 180)));
        }

        while lines.len() < available_rows {
            lines.push(Line::default());
        }

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Visualizer [Clock] ")
                .border_style(Style::default().fg(Color::Rgb(204, 156, 164))),
        );
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

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Visualizer [CMatrix] ")
                .border_style(Style::default().fg(Color::Rgb(0, 180, 0))),
        );
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
}
