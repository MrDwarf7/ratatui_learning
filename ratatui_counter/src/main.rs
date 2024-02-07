use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    prelude::{CrosstermBackend, Frame, Terminal},
    widgets::Paragraph,
};

use anyhow::Result;

struct App {
    counter: i64,
    should_quit: bool,
}

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

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui_draw(app: &App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            "You can use 'j' to decrement, 'k' to increment, and q to quit
Counter: {}",
            app.counter
        )),
        f.size(),
    )
}

fn ui_update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('k') => app.counter += 1,
                    Char('j') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
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

    let mut app = App {
        counter: 0,
        should_quit: false,
    };

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
