use crate::App;
use crate::popup::Popup;
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
        if modifiers == KeyModifiers::CONTROL {
            match key {
                KeyCode::Char('c') => sender.send(Bytes::from(vec![3])),
                KeyCode::Char('v') => sender.send(Bytes::from(vec![22])),
                KeyCode::Char('d') => sender.send(Bytes::from(vec![4])),
                KeyCode::Char('z') => sender.send(Bytes::from(vec![26])),
                KeyCode::Char('l') => sender.send(Bytes::from(vec![12])),
                KeyCode::Char('h') => {
                    self.active_popup = Some(Popup::Help);
                    Ok(())
                }
                _ => Ok(()),
            }
        } else {
            match key {
                KeyCode::Char(input) => sender.send(Bytes::from(input.to_string().into_bytes())),
                KeyCode::Backspace => sender.send(Bytes::from(vec![8])),
                KeyCode::Enter => {
                    #[cfg(unix)]
                    sender.send(Bytes::from(vec![b'\n']))?;
                    #[cfg(windows)]
                    sender.send(Bytes::from(vec![b'\r', b'\n']))?;
                    Ok(())
                }
                KeyCode::Left => sender.send(Bytes::from(vec![27, 91, 68])),
                KeyCode::Right => sender.send(Bytes::from(vec![27, 91, 67])),
                KeyCode::Up => sender.send(Bytes::from(vec![27, 91, 65])),
                KeyCode::Down => sender.send(Bytes::from(vec![27, 91, 66])),
                KeyCode::Home => sender.send(Bytes::from(vec![27, 91, 72])),
                KeyCode::End => sender.send(Bytes::from(vec![27, 91, 70])),
                KeyCode::PageUp => sender.send(Bytes::from(vec![27, 91, 53, 126])),
                KeyCode::PageDown => sender.send(Bytes::from(vec![27, 91, 54, 126])),
                KeyCode::Tab => sender.send(Bytes::from(vec![9])),
                KeyCode::BackTab => sender.send(Bytes::from(vec![27, 91, 90])),
                KeyCode::Delete => sender.send(Bytes::from(vec![27, 91, 51, 126])),
                KeyCode::Insert => sender.send(Bytes::from(vec![27, 91, 50, 126])),
                KeyCode::Esc => sender.send(Bytes::from(vec![27])),
                _ => Ok(()),
            }
        }
    }
}
