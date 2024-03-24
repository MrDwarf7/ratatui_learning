use std::rc::Rc;

use crate::app::{App, CurrentScreen, CurrentlyEditing};
pub use crate::constants::*;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub struct RenderBuilder {}

impl RenderBuilder {
    pub fn render_title(f: &mut Frame, app: &App, chunks: Rc<[Rect]>, index: usize) {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(match app.current_screen {
                CurrentScreen::Main => PRIMARY_COLOR,
                CurrentScreen::Editing => ACCENT_COLOR,
                CurrentScreen::Exiting => ERROR_COLOR,
            }));

        let title = Paragraph::new(Text::styled(
            "Create a JSON file",
            Style::default().fg(match app.current_screen {
                CurrentScreen::Main => PRIMARY_COLOR,
                CurrentScreen::Editing => ACCENT_COLOR,
                CurrentScreen::Exiting => ERROR_COLOR,
            }),
        ))
        .block(title_block);

        f.render_widget(title, chunks[index]);
    }

    pub fn render_body_list(f: &mut Frame, app: &App, chunks: Rc<[Rect]>, index: usize) {
        let mut list_items = Vec::<ListItem>::new();

        for key in app.pairs.keys() {
            list_items.push(ListItem::new(
                Line::from(Span::styled(
                    format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
                    Style::default()
                        .fg(LIST_ITEM_TEXT_COLOR)
                        .bg(LIST_ITEM_BG_COLOR),
                ))
                .style(
                    Style::default()
                        .fg(LIST_ITEM_TEXT_COLOR)
                        .bg(LIST_ITEM_BG_COLOR),
                ),
            ))
        }

        let list = List::new(list_items);

        f.render_widget(list, chunks[index]);
    }

    pub fn render_nav_text(app: &App) -> Vec<Span> {
        vec![
            match app.current_screen {
                CurrentScreen::Main => {
                    Span::styled("Normal Mode", Style::default().fg(HINT_NORMAL_MODE))
                }
                CurrentScreen::Editing => {
                    Span::styled("Editing Mode", Style::default().fg(HINT_EDITING_MODE))
                }
                CurrentScreen::Exiting => {
                    Span::styled("Exiting Mode", Style::default().fg(HINT_EXITING_MODE))
                }
            }
            .to_owned(),
            Span::styled(" | ", Style::default().fg(TEXT_COLOR)),
            {
                if let Some(editing) = &app.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            Span::styled(HINT_TEXT_KEY, Style::default().fg(DARK_TEXT_COLOR))
                        }
                        CurrentlyEditing::Value => {
                            Span::styled(HINT_TEXT_VALUE, Style::default().fg(DARK_TEXT_COLOR))
                        }
                    }
                } else {
                    Span::styled("Not editing anything", Style::default().fg(TEXT_COLOR))
                }
            },
        ]
    }

    pub fn render_hints_text(app: &App) -> Vec<Span> {
        vec![match app.current_screen {
            CurrentScreen::Main => Span::styled(HINT_TEXT, Style::default().fg(DARK_TEXT_COLOR)),
            CurrentScreen::Editing => {
                Span::styled(HINT_TEXT_EDITING, Style::default().fg(TEXT_COLOR))
            }
            CurrentScreen::Exiting => {
                Span::styled(HINT_TEXT_EXITING, Style::default().fg(TEXT_COLOR))
            }
        }
        .to_owned()]
    }

    pub fn render_footer_setup(
        current_keys_hint: Vec<Span>,
        chunks: Rc<[Rect]>,
        index: usize,
    ) -> (Paragraph, Rc<[Rect]>) {
        let key_note_footer = Paragraph::new(Line::from(current_keys_hint)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(DARK_TEXT_COLOR)),
        );

        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(chunks[index]);

        (key_note_footer, footer_chunks)
    }

    pub fn render_editing_popup(
        f: &mut Frame,
        app: &App,
        key_text_index: usize,
        value_text_index: usize,
    ) {
        if let Some(editing) = &app.currently_editing {
            let popup_block = Block::default()
                .title("Enter a new (key, value) pair")
                .borders(Borders::NONE)
                .style(Style::default().fg(PRIMARY_COLOR).bg(GENERAL_BG_COLOR));

            let area = centered_rect(60, 35, f.size());
            f.render_widget(popup_block, area);

            let popup_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

            let mut key_block = Block::default().title("Key").borders(Borders::ALL);
            let mut value_block = Block::default().title("Value").borders(Borders::ALL);

            let active_block_style = Style::default()
                .bg(match editing {
                    CurrentlyEditing::Key => ACTIVE_BG_COLOR,
                    CurrentlyEditing::Value => ACTIVE_BG_COLOR,
                })
                .fg(TEXT_COLOR);

            match editing {
                CurrentlyEditing::Key => key_block = key_block.style(active_block_style),
                CurrentlyEditing::Value => value_block = value_block.style(active_block_style),
            };

            let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
            f.render_widget(key_text, popup_chunks[key_text_index]);

            let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
            f.render_widget(value_text, popup_chunks[value_text_index]);
        }
    }

    pub fn render_exiting_popup(f: &mut Frame) {
        f.render_widget(Clear, f.size());
        let popup_block = Block::default()
            .title("[Y]es/[N]o")
            .borders(Borders::ALL)
            .style(Style::default().fg(TEXT_COLOR).bg(GENERAL_BG_COLOR))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(PRIMARY_COLOR));

        let exit_text = Text::styled(
            "Would you like to output the buffer to stdout?",
            Style::default().fg(ERROR_COLOR),
        );

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(40, 25, f.size());
        f.render_widget(exit_paragraph, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
