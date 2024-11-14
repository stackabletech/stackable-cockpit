use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers};
use snafu::{ResultExt, Snafu};
use tokio::sync::mpsc::Sender;

use super::state::{Message, Model};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read event"))]
    ReadEvent { source: std::io::Error },
}

pub async fn handle_event(_: &Model, message_tx: Sender<Message>) -> Result<(), Error> {
    if event::poll(Duration::from_millis(250)).context(ReadEventSnafu)? {
        if let Event::Key(key) = event::read().context(ReadEventSnafu)? {
            if key.kind == event::KeyEventKind::Press {
                for message in handle_key(key) {
                    message_tx.send(message).await.unwrap();
                }
            }
        }
    }

    Ok(())
}

pub fn handle_key(key: event::KeyEvent) -> Vec<Message> {
    let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
    let ctrl_pressed = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char('c') if ctrl_pressed => vec![Message::Quit],
        KeyCode::Char('q') /*| KeyCode::Esc*/ => vec![Message::Quit],
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
        },
        KeyCode::Tab if shift_pressed => vec![Message::PreviousTab],
        KeyCode::Tab => vec![Message::NextTab],
        _ => vec![],
    }
}
