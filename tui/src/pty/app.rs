use crate::App;
use bytes::Bytes;
use crossterm::event::{KeyCode, KeyModifiers};

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
        sender: &tokio::sync::mpsc::Sender<Bytes>,
    ) -> Result<(), String> {
        let bytes_to_send = if modifiers.contains(KeyModifiers::CONTROL) {
            match key {
                KeyCode::Char(c) if c.is_ascii_alphabetic() => {
                    let byte = (c.to_ascii_lowercase() as u8) & 0x1F;
                    vec![byte]
                }
                KeyCode::Char('[') => vec![27],
                KeyCode::Char('\\') => vec![28],
                KeyCode::Char(']') => vec![29],
                KeyCode::Char('^') => vec![30],
                KeyCode::Char('_') => vec![31],
                KeyCode::Char('@') => vec![0],
                _ => vec![],
            }
        } else if modifiers.contains(KeyModifiers::ALT) {
            match key {
                KeyCode::Char(c) => {
                    let mut bytes = vec![27];
                    let mut buf = [0; 4];
                    bytes.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                    bytes
                }
                _ => vec![],
            }
        } else {
            match key {
                KeyCode::Enter => vec![13],
                KeyCode::Tab => vec![9],
                KeyCode::Backspace => vec![127],
                KeyCode::Esc => vec![27],

                KeyCode::Up => vec![27, 91, 65],
                KeyCode::Down => vec![27, 91, 66],
                KeyCode::Right => vec![27, 91, 67],
                KeyCode::Left => vec![27, 91, 68],

                KeyCode::Home => vec![27, 91, 72],
                KeyCode::End => vec![27, 91, 70],
                KeyCode::PageUp => vec![27, 91, 53, 126],
                KeyCode::PageDown => vec![27, 91, 54, 126],
                KeyCode::Insert => vec![27, 91, 50, 126],
                KeyCode::Delete => vec![27, 91, 51, 126],

                KeyCode::F(1) => vec![27, 79, 80],
                KeyCode::F(2) => vec![27, 79, 81],
                KeyCode::F(3) => vec![27, 79, 82],
                KeyCode::F(4) => vec![27, 79, 83],
                KeyCode::F(5) => vec![27, 91, 49, 53, 126],
                KeyCode::F(6) => vec![27, 91, 49, 55, 126],
                KeyCode::F(7) => vec![27, 91, 49, 56, 126],
                KeyCode::F(8) => vec![27, 91, 49, 57, 126],
                KeyCode::F(9) => vec![27, 91, 50, 48, 126],
                KeyCode::F(10) => vec![27, 91, 50, 49, 126],
                KeyCode::F(11) => vec![27, 91, 50, 51, 126],
                KeyCode::F(12) => vec![27, 91, 50, 52, 126],

                KeyCode::Char(c) => {
                    let mut buf = [0; 4];
                    c.encode_utf8(&mut buf).as_bytes().to_vec()
                }

                _ => vec![],
            }
        };

        if !bytes_to_send.is_empty() {
            sender
                .try_send(Bytes::from(bytes_to_send))
                .map_err(|e| format!("Channel error: {}", e))?;
        }
        Ok(())
    }
}
