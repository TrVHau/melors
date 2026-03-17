mod input;
mod render;
mod state;

use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::app::App;

use self::state::{UiState, VisualizerMode};

fn record_first_error<E>(slot: &mut Option<anyhow::Error>, result: std::result::Result<(), E>)
where
    E: Into<anyhow::Error>,
{
    if slot.is_some() {
        return;
    }

    if let Err(error) = result {
        *slot = Some(error.into());
    }
}

pub fn run(app: &mut App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut ui = UiState::new();

    let run_result = (|| -> Result<()> {
        loop {
            app.refresh_playback_position()?;
            if let Some(status) = app.poll_scan_status() {
                ui.status = status;
            }
            terminal.draw(|frame| ui.draw(frame, app))?;

            let poll_ms = match ui.visualizer_mode {
                VisualizerMode::Cava => 33,
                VisualizerMode::CMatrix => 66,
                VisualizerMode::Clock => 1000,
            };

            if event::poll(Duration::from_millis(poll_ms))?
                && let Event::Key(key) = event::read()?
                && ui.handle_key(app, key)?
            {
                return Ok(());
            }
        }
    })();

    let mut first_error = run_result.err();

    record_first_error(&mut first_error, disable_raw_mode());
    record_first_error(
        &mut first_error,
        execute!(terminal.backend_mut(), LeaveAlternateScreen),
    );
    record_first_error(&mut first_error, terminal.show_cursor());
    record_first_error(&mut first_error, app.persist_playback_state_now());

    match first_error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}
