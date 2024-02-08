use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            "
                Press 'Esc', 'Ctrl+c, or 'q' to stop running.\n\
                Press 'j' and 'k' to decrement and increment the counter respectively.\n\
                Counter: {}
                ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter application")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Red).bg(Color::Black))
        .alignment(Alignment::Center),
        f.size(),
    )
}
