extern crate crossterm;
extern crate ratatui;

mod app;
mod constants;
mod rendering;
mod screen_handlers;
mod ui;

use app::{App, CurrentScreen, CurrentlyEditing};
use crossterm::{
    event::{
        //
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
        KeyEventKind,
    },
    execute,
    terminal::{
        //
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use screen_handlers::{
    //
    screen_handler_editing,
    screen_handler_exiting,
    screen_handler_main,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stderr = std::io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> std::io::Result<bool> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skipping events taht aren't of type Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => screen_handler_main(&key, app),
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => match screen_handler_exiting(&key) {
                        Ok(res) => return Ok(res),
                        Err(err) => return Err(err),
                    },
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                    screen_handler_editing(&key, app)
                }
                _ => {}
            }
        }
    }
}
