use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers};
use snafu::{ResultExt, Snafu};
use tokio::sync::mpsc::Sender;

use super::{message::Message, state::Model};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read event"))]
    ReadEvent { source: std::io::Error },
}

pub async fn handle_event(_: &Model, message_tx: Sender<Message>) -> Result<(), Error> {
    if event::poll(Duration::from_millis(250)).context(ReadEventSnafu)? {
        if let Event::Key(key) = event::read().context(ReadEventSnafu)? {
            if key.kind == event::KeyEventKind::Press {
                let maybe_message = handle_key(key);
                if let Some(message) = maybe_message {
                    message_tx.send(message).await.unwrap();
                }
            }
        }
    }

    Ok(())
}

pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
    let _shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
    let ctrl_pressed = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char('c') if ctrl_pressed => Some(Message::Quit),
        KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),
        KeyCode::Char('j') | KeyCode::Down => todo!(),
        KeyCode::Char('k') | KeyCode::Up => todo!(),
        _ => None,
    }
}
