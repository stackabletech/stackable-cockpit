use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers};
use snafu::{ResultExt, Snafu};
use tokio::sync::mpsc::Sender;
use tui_logger::TuiWidgetEvent;

use super::{
    rendering::tabs::SelectedTab,
    state::{Message, Model},
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read event"))]
    ReadEvent { source: std::io::Error },
}

pub async fn handle_event(model: &Model, message_tx: Sender<Message>) -> Result<(), Error> {
    if event::poll(Duration::from_millis(250)).context(ReadEventSnafu)? {
        if let Event::Key(key) = event::read().context(ReadEventSnafu)? {
            if key.kind == event::KeyEventKind::Press {
                for message in handle_key(model, key) {
                    message_tx.send(message).await.unwrap();
                }
            }
        }
    }

    Ok(())
}

pub fn handle_key(model: &Model, key: event::KeyEvent) -> Vec<Message> {
    let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
    let ctrl_pressed = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char('c') if ctrl_pressed => { return vec![Message::Quit]; }
        KeyCode::Char('q') /*| KeyCode::Esc*/ => { return vec![Message::Quit]; }
        KeyCode::Tab if shift_pressed => { return vec![Message::PreviousTab]; }
        KeyCode::Tab => { return vec![Message::NextTab]; }
        _ => {},
    };

    match model.selected_tab {
        SelectedTab::Stacklets => handle_key_in_stacklets(key),
        SelectedTab::Logs => handle_key_in_logs(key),
    }
}

pub fn handle_key_in_stacklets(key: event::KeyEvent) -> Vec<Message> {
    match key.code {
        KeyCode::Char('k') | KeyCode::Up => vec![Message::StackletListUp { steps: 1 }],
        KeyCode::Char('j') | KeyCode::Down => vec![Message::StackletListDown { steps: 1 }],
        KeyCode::Home => vec![Message::StackletListStart],
        KeyCode::End => vec![Message::StackletListEnd],
        KeyCode::PageUp => {
            // FIXME: Determine actual table height
            let table_height = 10;
            vec![Message::StackletListUp {
                steps: table_height,
            }]
        }
        KeyCode::PageDown => {
            // FIXME: Determine actual table height
            let table_height = 10;
            vec![Message::StackletListDown {
                steps: table_height,
            }]
        }
        _ => vec![],
    }
}

pub fn handle_key_in_logs(key: event::KeyEvent) -> Vec<Message> {
    match key.code {
        KeyCode::Char('k') | KeyCode::Up => {
            vec![Message::LoggingWidgetMessage(TuiWidgetEvent::UpKey)]
        }
        KeyCode::Char('j') | KeyCode::Down => {
            vec![Message::LoggingWidgetMessage(TuiWidgetEvent::DownKey)]
        }
        KeyCode::Char('h') | KeyCode::Left => {
            vec![Message::LoggingWidgetMessage(TuiWidgetEvent::LeftKey)]
        }
        KeyCode::Char('l') | KeyCode::Right => {
            vec![Message::LoggingWidgetMessage(TuiWidgetEvent::RightKey)]
        }
        KeyCode::PageUp => {
            vec![Message::LoggingWidgetMessage(TuiWidgetEvent::PrevPageKey)]
        }
        KeyCode::PageDown => {
            vec![Message::LoggingWidgetMessage(TuiWidgetEvent::NextPageKey)]
        }
        KeyCode::Esc | KeyCode::End => {
            vec![Message::LoggingWidgetMessage(TuiWidgetEvent::EscapeKey)]
        }
        _ => vec![],
    }
}
