use crate::App;
use bytes::Bytes;
use crossterm::event::{KeyCode, KeyModifiers};
use std::sync::mpsc::Sender;

impl App {
    pub fn handle_popup_key(
        &mut self,
        key: KeyCode,
    ) -> Result<(), std::sync::mpsc::SendError<Bytes>> {
        match key {
            KeyCode::Enter => {
                self.active_popup = None;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn handle_terminal_key(
        &mut self,
        key: KeyCode,
        modifiers: KeyModifiers,
        sender: &Sender<Bytes>,
    ) -> Result<(), std::sync::mpsc::SendError<Bytes>> {
        if modifiers.contains(KeyModifiers::CONTROL) {
            match key {
                KeyCode::Char(c) if c.is_ascii_alphabetic() => {
                    let byte = (c.to_ascii_lowercase() as u8) & 0x1F;
                    sender.send(Bytes::from(vec![byte]))
                }
                KeyCode::Char('[') => sender.send(Bytes::from(vec![27])),
                KeyCode::Char('\\') => sender.send(Bytes::from(vec![28])),
                KeyCode::Char(']') => sender.send(Bytes::from(vec![29])),
                KeyCode::Char('^') => sender.send(Bytes::from(vec![30])),
                KeyCode::Char('_') => sender.send(Bytes::from(vec![31])),
                KeyCode::Char('@') => sender.send(Bytes::from(vec![0])),
                _ => Ok(()),
            }
        } else if modifiers.contains(KeyModifiers::ALT) {
            match key {
                KeyCode::Char(c) => {
                    let mut bytes = vec![27];
                    bytes.extend_from_slice(c.to_string().as_bytes());
                    sender.send(Bytes::from(bytes))
                }
                _ => Ok(()),
            }
        } else {
            match key {
                KeyCode::Enter => sender.send(Bytes::from(vec![13])),
                KeyCode::Tab => sender.send(Bytes::from(vec![9])),
                KeyCode::Backspace => sender.send(Bytes::from(vec![127])),
                KeyCode::Esc => sender.send(Bytes::from(vec![27])),

                KeyCode::Up => sender.send(Bytes::from(vec![27, 91, 65])),
                KeyCode::Down => sender.send(Bytes::from(vec![27, 91, 66])),
                KeyCode::Right => sender.send(Bytes::from(vec![27, 91, 67])),
                KeyCode::Left => sender.send(Bytes::from(vec![27, 91, 68])),

                KeyCode::Home => sender.send(Bytes::from(vec![27, 91, 72])),
                KeyCode::End => sender.send(Bytes::from(vec![27, 91, 70])),
                KeyCode::PageUp => sender.send(Bytes::from(vec![27, 91, 53, 126])),
                KeyCode::PageDown => sender.send(Bytes::from(vec![27, 91, 54, 126])),
                KeyCode::Insert => sender.send(Bytes::from(vec![27, 91, 50, 126])),
                KeyCode::Delete => sender.send(Bytes::from(vec![27, 91, 51, 126])),

                KeyCode::F(1) => sender.send(Bytes::from(vec![27, 79, 80])),
                KeyCode::F(2) => sender.send(Bytes::from(vec![27, 79, 81])),
                KeyCode::F(3) => sender.send(Bytes::from(vec![27, 79, 82])),
                KeyCode::F(4) => sender.send(Bytes::from(vec![27, 79, 83])),
                KeyCode::F(5) => sender.send(Bytes::from(vec![27, 91, 49, 53, 126])),
                KeyCode::F(6) => sender.send(Bytes::from(vec![27, 91, 49, 55, 126])),
                KeyCode::F(7) => sender.send(Bytes::from(vec![27, 91, 49, 56, 126])),
                KeyCode::F(8) => sender.send(Bytes::from(vec![27, 91, 49, 57, 126])),
                KeyCode::F(9) => sender.send(Bytes::from(vec![27, 91, 50, 48, 126])),
                KeyCode::F(10) => sender.send(Bytes::from(vec![27, 91, 50, 49, 126])),
                KeyCode::F(11) => sender.send(Bytes::from(vec![27, 91, 50, 51, 126])),
                KeyCode::F(12) => sender.send(Bytes::from(vec![27, 91, 50, 52, 126])),

                KeyCode::Char(c) => sender.send(Bytes::from(c.to_string().into_bytes())),

                _ => Ok(()),
            }
        }
    }
}
