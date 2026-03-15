use super::*;

impl UiState {
    pub fn handle_key(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        if key.kind != event::KeyEventKind::Press {
            return Ok(false);
        }

        if self.handle_alt_shortcuts(key) {
            return Ok(false);
        }

        match self.mode {
            InputMode::Search => return self.handle_search_input(app, key),
            InputMode::Rename => return self.handle_rename_input(app, key),
            InputMode::EditTag => return self.handle_edit_tag_input(app, key),
            InputMode::Normal => {}
        }

        self.handle_normal_key(app, key)
    }

    fn handle_alt_shortcuts(&mut self, key: KeyEvent) -> bool {
        if !key.modifiers.contains(KeyModifiers::ALT) {
            return false;
        }

        match key.code {
            KeyCode::Char('1') => {
                self.set_visualizer_mode(VisualizerMode::Cava);
                self.status = String::from("Visualizer: Cava");
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
