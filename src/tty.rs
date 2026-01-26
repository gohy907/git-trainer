use crate::App;
use crate::task::Task;
use crate::ui;
use bytes::Bytes;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::DefaultTerminal;
use std::{
    io::{self},
    sync::{Arc, RwLock, mpsc::Sender},
    time::Duration,
};
use thiserror::Error;
use tui_term::vt100;

use bollard::container::LogOutput;
use futures::StreamExt;
use tokio::io::AsyncWriteExt;

use crate::docker::{self, resize_container};

#[derive(Debug)]
struct Size {
    cols: u16,
    rows: u16,
}

#[derive(Debug, Error)]
pub enum PreparePtyError {
    #[error("While working with Docker: {0}")]
    DockerError(#[from] bollard::errors::Error),

    #[error("IO error: {0}")]
    DrawTerminalError(#[from] io::Error),

    #[error("While running PTY: {0}")]
    RunPtyError(#[from] RunPtyError),

    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

#[derive(Debug, Error)]
pub enum RunPtyError {
    #[error("While working with Docker: {0}")]
    DockerError(#[from] bollard::errors::Error),

    #[error("IO error: {0}")]
    DrawTerminalError(#[from] io::Error),

    #[error("While sending to PTY: {0}")]
    MPSCError(#[from] std::sync::mpsc::SendError<bytes::Bytes>),

    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

impl App {
    pub async fn prepare_pty_bollard(
        terminal: &mut DefaultTerminal,
        task: &Task,
    ) -> Result<(), PreparePtyError> {
        let mut handles = Vec::new();
        let size = Size {
            rows: terminal.size()?.height - 4,
            cols: terminal.size()?.width - 2,
        };

        docker::start_container(task).await?;

        let container_name = task.container_name();
        docker::resize_container(container_name.clone(), size.rows as i32, size.cols as i32)
            .await?;
        let res = docker::attach_container(task).await?;

        let mut output_stream = res.output;
        let mut input = res.input;

        {
            let rows = size.rows as i32;
            let cols = size.cols as i32;
            let container_name = task.container_name();
            let handle = tokio::spawn(async move {
                let _ = resize_container(container_name, rows, cols).await;
            });
            handles.push(handle);
        }

        let parser = Arc::new(RwLock::new(vt100::Parser::new(size.rows, size.cols, 0)));

        let (tx, rx) = std::sync::mpsc::channel::<Bytes>();
        let (exit_tx, exit_rx) = std::sync::mpsc::channel::<()>();

        {
            let parser = parser.clone();
            let exit_tx = exit_tx.clone();
            let handle = tokio::spawn(async move {
                let mut buf = Vec::new();
                while let Some(item) = output_stream.next().await {
                    match item {
                        Ok(log) => {
                            let bytes: &[u8] = match log {
                                LogOutput::StdOut { ref message } => message.as_ref(),
                                LogOutput::StdErr { ref message } => message.as_ref(),
                                LogOutput::StdIn { ref message } => message.as_ref(),
                                LogOutput::Console { ref message } => message.as_ref(),
                            };

                            if !bytes.is_empty() {
                                buf.extend_from_slice(bytes);
                                if let Ok(mut p) = parser.write() {
                                    p.process(&buf);
                                }
                                buf.clear();
                            }
                        }
                        Err(e) => {
                            eprintln!("docker output error: {e}");
                            let _ = exit_tx.send(());
                            break;
                        }
                    }
                }
                let _ = exit_tx.send(());
            });
            handles.push(handle);
        }

        // Writer таск
        let handle = tokio::spawn(async move {
            while let Ok(bytes) = rx.recv() {
                if input.write_all(&bytes).await.is_err() {
                    break;
                }
                if input.flush().await.is_err() {
                    break;
                }
            }
        });
        handles.push(handle);

        Self::run_pty_bollard(terminal, parser, tx, exit_rx, container_name).await?;

        for handle in handles {
            handle.await.map_err(PreparePtyError::JoinError)?;
        }
        Ok(())
    }

    pub async fn run_pty_bollard(
        terminal: &mut DefaultTerminal,
        parser: Arc<RwLock<vt100::Parser>>,
        sender: Sender<Bytes>,
        exit_rx: std::sync::mpsc::Receiver<()>,
        container_name: String,
    ) -> Result<(), RunPtyError> {
        let mut handles = Vec::new();
        loop {
            if exit_rx.try_recv().is_ok() {
                for handle in handles {
                    if let Err(e) = handle.await {
                        return Err(RunPtyError::JoinError(e));
                    }
                }
                return Ok(());
            }

            terminal.draw(|f| ui::ui_pty(f, parser.read().unwrap().screen()))?;

            if event::poll(Duration::from_millis(10))? {
                match event::read()? {
                    Event::Key(key) => {
                        if key.kind == KeyEventKind::Press {
                            if key.modifiers == KeyModifiers::CONTROL {
                                match key.code {
                                    KeyCode::Char('c') => sender.send(Bytes::from(vec![3]))?,
                                    KeyCode::Char('v') => sender.send(Bytes::from(vec![22]))?,
                                    KeyCode::Char('d') => sender.send(Bytes::from(vec![4]))?,
                                    KeyCode::Char('z') => sender.send(Bytes::from(vec![26]))?,
                                    KeyCode::Char('l') => sender.send(Bytes::from(vec![12]))?,
                                    _ => {}
                                }
                            } else {
                                match key.code {
                                    KeyCode::Char(input) => {
                                        sender.send(Bytes::from(input.to_string().into_bytes()))?
                                    }
                                    KeyCode::Backspace => {
                                        sender.send(Bytes::from(vec![8]))?;
                                    }
                                    KeyCode::Enter => {
                                        #[cfg(unix)]
                                        sender.send(Bytes::from(vec![b'\n']))?;
                                        #[cfg(windows)]
                                        sender.send(Bytes::from(vec![b'\r', b'\n']))?;
                                    }
                                    KeyCode::Left => sender.send(Bytes::from(vec![27, 91, 68]))?,
                                    KeyCode::Right => sender.send(Bytes::from(vec![27, 91, 67]))?,
                                    KeyCode::Up => sender.send(Bytes::from(vec![27, 91, 65]))?,
                                    KeyCode::Down => sender.send(Bytes::from(vec![27, 91, 66]))?,
                                    KeyCode::Home => sender.send(Bytes::from(vec![27, 91, 72]))?,
                                    KeyCode::End => sender.send(Bytes::from(vec![27, 91, 70]))?,
                                    KeyCode::PageUp => {
                                        sender.send(Bytes::from(vec![27, 91, 53, 126]))?
                                    }
                                    KeyCode::PageDown => {
                                        sender.send(Bytes::from(vec![27, 91, 54, 126]))?
                                    }
                                    KeyCode::Tab => sender.send(Bytes::from(vec![9]))?,
                                    KeyCode::BackTab => {
                                        sender.send(Bytes::from(vec![27, 91, 90]))?
                                    }
                                    KeyCode::Delete => {
                                        sender.send(Bytes::from(vec![27, 91, 51, 126]))?
                                    }
                                    KeyCode::Insert => {
                                        sender.send(Bytes::from(vec![27, 91, 50, 126]))?
                                    }
                                    KeyCode::Esc => {
                                        sender.send(Bytes::from(vec![27]))?;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Event::Resize(cols, rows) => {
                        let rows = rows - 4;
                        let cols = cols - 2;
                        parser.write().unwrap().set_size(rows, cols);
                        let name = container_name.clone();

                        let handle = tokio::spawn(async move {
                            let _ = resize_container(name, rows as i32, cols as i32).await;
                        });
                        handles.push(handle);
                    }
                    _ => {}
                }
            }
        }
    }
}
