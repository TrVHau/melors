mod app;
mod core;
mod features;
mod quality;
mod services;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
    let mut app = app::App::boot()?;
    app.run_tui()
}
