use crate::Frame;
use crate::app::{App, VERSION};
use crate::docker::{self, ensure_task_container_running};
use crate::docker::{CmdOutput, resize_container};
use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
use ratatui::layout::{Alignment, Constraint};
use ratatui::prelude::{Direction, Layout};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use tui_term::{vt100, widget::PseudoTerminal};
use vt100::Screen;

use crate::db::Task;
use bytes::Bytes;
use ratatui::DefaultTerminal;
use std::{
    io,
    sync::{Arc, RwLock, mpsc::Sender},
};

use bollard::container::LogOutput;
use futures::StreamExt;
use thiserror::Error;
use tokio::io::AsyncWriteExt;

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

pub enum PtyExitStatus {
    Exit,
    RestartTask,
}

impl App {
    pub fn render_pty(&mut self, frame: &mut Frame, screen: &Screen) {
        let title = Line::from(format!("git-trainer v{}", VERSION).bold()).centered();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(frame.area());
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().add_modifier(Modifier::BOLD));
        let pseudo_term = PseudoTerminal::new(screen).block(block);
        let explanation = "Напишите команду exit для выхода".to_string();
        let explanation = Paragraph::new(explanation)
            .style(Style::default())
            .alignment(Alignment::Center);
        frame.render_widget(title, chunks[0]);
        frame.render_widget(pseudo_term, chunks[1]);
        frame.render_widget(explanation, chunks[2]);

        if let Some(popup) = &self.active_popup {
            popup.render(frame, self);
        }
    }

    pub async fn prepare_pty_bollard(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<PtyExitStatus, PreparePtyError> {
        let task = self.task_under_cursor();
        let mut handles = Vec::new();
        let size = Size {
            rows: terminal.size()?.height - 4,
            cols: terminal.size()?.width - 2,
        };

        ensure_task_container_running(task).await?;
        let container_name = task.container_name.clone();
        docker::resize_container(container_name.clone(), size.rows as i32, size.cols as i32)
            .await?;
        let res = docker::attach_container(task).await?;

        let mut output_stream = res.output;
        let mut input = res.input;

        {
            let rows = size.rows as i32;
            let cols = size.cols as i32;
            let container_name = task.container_name.clone();
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
                use std::io::Write;

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
                                if let Ok(mut p) = parser.write() {
                                    let _ = p.write_all(bytes);
                                }
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

        let exit_status = self
            .run_pty_bollard(terminal, parser, tx, exit_rx, container_name)
            .await?;

        for handle in handles {
            handle.await.map_err(PreparePtyError::JoinError)?;
        }
        Ok(exit_status)
    }

    async fn run_pty_bollard(
        &mut self,
        terminal: &mut DefaultTerminal,
        parser: Arc<RwLock<vt100::Parser>>,
        sender: Sender<Bytes>,
        exit_rx: std::sync::mpsc::Receiver<()>,
        container_name: String,
    ) -> Result<PtyExitStatus, RunPtyError> {
        let mut handles = Vec::new();
        loop {
            let task = self.task_under_cursor();
            let a = docker::exec_command(&task, "cat /etc/git-trainer/status")
                .await
                .unwrap_or(CmdOutput {
                    output: "error".to_string(),
                    exit_code: -1,
                })
                .output;
            if a == "1".to_string() {
                let exit_command = Bytes::from("exit\n");
                sender.send(exit_command)?;

                for handle in handles.drain(..) {
                    if let Err(e) = handle.await {
                        return Err(RunPtyError::JoinError(e));
                    }
                }
                return Ok(PtyExitStatus::RestartTask);
            } else if a == "2".to_string() {
                let task = self.task_under_cursor();
                // самый костыльный костыль. Миша, если ты это читаешь, пойми и прости меня.
                let _ = docker::exec_command(task, "git-trainer task").await;

                self.test_submitted_task().await;
                self.update_context();
            }
            if exit_rx.try_recv().is_ok() {
                for handle in handles.drain(..) {
                    if let Err(e) = handle.await {
                        return Err(RunPtyError::JoinError(e));
                    }
                }
                return Ok(PtyExitStatus::Exit);
            }

            terminal.draw(|f| self.render_pty(f, parser.read().unwrap().screen()))?;

            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        if let Some(_) = self.active_popup {
                            let _ = self.handle_popup_key(key.code);
                        } else {
                            let _ = self.handle_terminal_key(key.code, key.modifiers, &sender);
                        }
                    }
                }
                Event::Resize(cols, rows) => {
                    let rows = rows - 4;
                    let cols = cols - 2;
                    parser.write().unwrap().screen_mut().set_size(rows, cols);
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
