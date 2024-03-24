use crate::{App, CurrentScreen, CurrentlyEditing};
use crossterm::event::{KeyCode, KeyEvent};
use std::io;

pub fn screen_handler_main(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') => {
            app.current_screen = CurrentScreen::Exiting;
        }
        KeyCode::Char('e') => {
            app.current_screen = CurrentScreen::Editing;
            app.currently_editing = Some(CurrentlyEditing::Key);
        }
        _ => {}
    }
}

pub fn screen_handler_exiting(key: &KeyEvent) -> io::Result<bool> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => Ok(true),
        KeyCode::Char('n') | KeyCode::Char('q') => Ok(false),
        _ => Ok(false),
    }
}

pub fn screen_handler_editing(key: &KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Enter => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        app.currently_editing = Some(CurrentlyEditing::Value);
                    }
                    CurrentlyEditing::Value => {
                        app.save_key_value();
                        app.current_screen = CurrentScreen::Main;
                    }
                }
            }
        }
        KeyCode::Backspace => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        app.key_input.pop();
                    }
                    CurrentlyEditing::Value => {
                        app.value_input.pop();
                    }
                }
            }
        }
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::Main;
            app.currently_editing = None;
        }
        KeyCode::Tab => {
            app.toggle_editing();
        }
        KeyCode::Char(value) => {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        app.key_input.push(value);
                    }
                    CurrentlyEditing::Value => {
                        app.value_input.push(value);
                    }
                }
            }
        }
        _ => {}
    }
}
