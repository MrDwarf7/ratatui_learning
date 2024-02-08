use anyhow::Result;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Frame, Terminal},
    // widgets::Paragraph,
};

pub mod app;

use app::App;

pub mod event;
pub mod ui;

// pub mod tui;
// pub mod update;

const POLL_RATE: std::time::Duration = std::time::Duration::from_millis(1000);

fn main() -> Result<()> {
    startup()?;
    let status = run();
    shutdown()?;
    status?;
    Ok(())
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn ui_draw(app: &App, f: &mut Frame) {}

fn ui_update(app: &mut App) -> Result<()> {
    if event::poll(POLL_RATE)? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('k') => app.increment_counter(),
                    Char('j') => app.decrement_counter(),
                    Char('q') => app.quit(),
                    // Key(KeyEvent("Esc") => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut app = App::default();

    loop {
        t.draw(|f| {
            ui_draw(&app, f);
        })?;

        ui_update(&mut app)?;

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
