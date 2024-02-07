use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

use std::io::stdout;

use anyhow::Result;

fn main() -> Result<()> {
    // let stdout: std::io::Stdout = stdout();
    // let mut handle = stdout.lock();

    stdout().execute(EnterAlternateScreen)?;

    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        // TODO: Draw UI

        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello world, but from the TUI baby! (press q to quit btw)")
                    .black()
                    .bold()
                    .on_red(),
                area,
            );
        });

        // TODO: Handle events

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')
                    || key.code == KeyCode::Esc
                {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

// fn draw_ui() {
//     // TODO: Migrate logic here
// }
