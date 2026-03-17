use super::*;

impl UiState {
    pub fn handle_key(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        let started = std::time::Instant::now();
        if key.kind != event::KeyEventKind::Press {
            return Ok(false);
        }

        if self.handle_alt_shortcuts(key) {
            return Ok(false);
        }

        let should_quit = match self.mode {
            InputMode::Search => self.handle_search_input(app, key)?,
            InputMode::EditTag => self.handle_edit_tag_input(app, key)?,
            InputMode::PlaylistModal => self.handle_playlist_modal_input(app, key)?,
            InputMode::Normal => self.handle_normal_key(app, key)?,
        };
        self.record_ui_action_latency(started.elapsed().as_micros());
        Ok(should_quit)
    }

    fn handle_alt_shortcuts(&mut self, key: KeyEvent) -> bool {
        if !key.modifiers.contains(KeyModifiers::ALT) {
            return false;
        }

        match key.code {
            KeyCode::Char('0') => {
                self.set_visualizer_mode(VisualizerMode::Off);
                self.status = String::from("Visualizer: Off (low power)");
                true
            }
            KeyCode::Char('1') => {
                self.set_visualizer_mode(VisualizerMode::Cava);
                self.status = String::from("Visualizer: Demo Bars");
                true
            }
            KeyCode::Char('2') => {
                self.set_visualizer_mode(VisualizerMode::Clock);
                self.status = String::from("Visualizer: Clock");
                true
            }
            KeyCode::Char('3') => {
                self.set_visualizer_mode(VisualizerMode::CMatrix);
                self.status = String::from("Visualizer: CMatrix");
                true
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                let theme = self.cycle_theme();
                self.status = format!("Theme: {}", theme);
                true
            }
            _ => false,
        }
    }
}
