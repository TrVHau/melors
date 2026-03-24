mod input;
mod render;
mod state;

use std::io;
use std::time::{Duration, Instant};

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
    let mut needs_redraw = true;
    let mut last_draw_at = Instant::now();

    let run_result = (|| -> Result<()> {
        loop {
            if let Some(status) = app.poll_scan_status() {
                ui.status = status;
                needs_redraw = true;
            }

            let frame_interval = match ui.visualizer_mode {
                VisualizerMode::Off => Duration::from_millis(1000),
                VisualizerMode::Cava => Duration::from_millis(33),
                VisualizerMode::CMatrix => Duration::from_millis(66),
                VisualizerMode::Clock => Duration::from_millis(1000),
            };
            let frame_interval = if ui.mode == self::state::InputMode::PlaylistModal {
                frame_interval.max(Duration::from_millis(120))
            } else {
                frame_interval
            };

            if needs_redraw || last_draw_at.elapsed() >= frame_interval {
                app.refresh_playback_position()?;
                terminal.draw(|frame| ui.draw(frame, app))?;
                last_draw_at = Instant::now();
                needs_redraw = false;
            }

            let time_until_frame = frame_interval.saturating_sub(last_draw_at.elapsed());
            let poll_timeout = time_until_frame.min(Duration::from_millis(40));

            if event::poll(poll_timeout)?
                && let Event::Key(key) = event::read()?
            {
                if ui.handle_key(app, key)? {
                    return Ok(());
                }
                needs_redraw = true;
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
