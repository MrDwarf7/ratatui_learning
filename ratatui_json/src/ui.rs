use crate::{
    app::{App, CurrentScreen},
    rendering::RenderBuilder,
};

pub use crate::constants::*;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    RenderBuilder::render_title(f, app, chunks.clone(), 0);

    RenderBuilder::render_body_list(f, app, chunks.clone(), 1);

    let current_nav_text = RenderBuilder::render_nav_text(app);

    let mode_footer = Paragraph::new(Line::from(current_nav_text)).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(DARK_TEXT_COLOR)),
    );

    let current_keys_hint = RenderBuilder::render_hints_text(app);

    let (key_notes_footer, footer_chunks) =
        RenderBuilder::render_footer_setup(current_keys_hint, chunks.clone(), 2);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    RenderBuilder::render_editing_popup(f, app, 0, 1);

    if let CurrentScreen::Exiting = app.current_screen {
        RenderBuilder::render_exiting_popup(f);
    }
}

// Util/Helper func
// fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
//     let popup_layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Percentage((100 - percent_y) / 2),
//             Constraint::Percentage(percent_y),
//             Constraint::Percentage((100 - percent_y) / 2),
//         ])
//         .split(r);

//     Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Percentage((100 - percent_x) / 2),
//             Constraint::Percentage(percent_x),
//             Constraint::Percentage((100 - percent_x) / 2),
//         ])
//         .split(popup_layout[1])[1]
// }
