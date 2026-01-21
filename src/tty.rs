use crate::App;
use crate::task::Task;
use crate::ui;
use bytes::Bytes;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use nix::libc;
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use ratatui::DefaultTerminal;
use std::sync::Mutex;
use std::{
    io::{self, Read, Write},
    sync::{Arc, RwLock, mpsc::Sender},
    time::Duration,
};
use tui_term::vt100;

#[derive(Debug)]
struct Size {
    cols: u16,
    rows: u16,
}

impl App {
    pub fn prepare_pty(terminal: &mut DefaultTerminal, task: &Task) -> std::io::Result<()> {
        let size = Size {
            rows: terminal.size()?.height - 5,
            cols: terminal.size()?.width,
        };

        let pty_system = NativePtySystem::default();
        let cwd = std::env::current_dir().unwrap();
        let mut cmd = CommandBuilder::new("docker");
        cmd.arg("run");
        cmd.arg("-it"); // Interactive + TTY - КРИТИЧНО!
        // cmd.arg("--rm"); // Удалить контейнер после выхода
        cmd.arg(task.image_name()); // Образ (замените на свой)
        cmd.arg("/bin/bash");
        cmd.cwd(cwd);

        let pair = pty_system
            .openpty(PtySize {
                rows: size.rows,
                cols: size.cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();
        // Wait for the child to complete
        std::thread::spawn(move || {
            let mut child = pair.slave.spawn_command(cmd).unwrap();
            let _child_exit_status = child.wait().unwrap();
            drop(pair.slave);
        });

        pair.master.try_clone_reader().unwrap();
        let parser = Arc::new(RwLock::new(vt100::Parser::new(size.rows, size.cols, 0)));
        let master = Arc::new(Mutex::new(pair.master));
        let master_for_reader = master.clone();
        let master_for_writer = master.clone();
        let master_for_run = master.clone();

        // Reader поток
        {
            let parser = parser.clone();
            std::thread::spawn(move || {
                let mut reader = master_for_reader
                    .lock()
                    .unwrap()
                    .try_clone_reader()
                    .unwrap();
                let mut buf = [0u8; 8192];
                let mut processed_buf = Vec::new();

                loop {
                    let sz = reader.read(&mut buf).unwrap();
                    if sz == 0 {
                        break;
                    }
                    if sz > 0 {
                        processed_buf.extend_from_slice(&buf[..sz]);
                        let mut parser = parser.write().unwrap();
                        parser.process(&processed_buf);
                        processed_buf.clear();
                    }
                }
            });
        }
        let (tx, rx) = std::sync::mpsc::channel::<Bytes>();
        // Writer поток
        std::thread::spawn(move || {
            let mut writer = master_for_writer.lock().unwrap().take_writer().unwrap();
            while let Ok(bytes) = rx.recv() {
                writer.write_all(&bytes).unwrap();
            }
            drop(writer);
        });

        let result = Self::run_pty(terminal, parser, tx, master_for_run);
        println!("{size:?}");
        result
    }

    pub fn run_pty(
        terminal: &mut DefaultTerminal,
        parser: Arc<RwLock<vt100::Parser>>,
        sender: Sender<Bytes>,
        master: Arc<Mutex<Box<dyn portable_pty::MasterPty + Send>>>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui::ui_pty(f, parser.read().unwrap().screen()))?;

            // Event read is blocking
            if event::poll(Duration::from_millis(10))? {
                // It's guaranteed that the `read()` won't block when the `poll()`
                // function returns `true`
                match event::read()? {
                    Event::Key(key) => {
                        if key.kind == KeyEventKind::Press {
                            if key.modifiers == KeyModifiers::CONTROL {
                                match key.code {
                                    KeyCode::Char('c') => {
                                        sender.send(Bytes::from(vec![3])).unwrap()
                                    }
                                    KeyCode::Char('v') => {
                                        sender.send(Bytes::from(vec![22])).unwrap()
                                    }
                                    KeyCode::Char('d') => {
                                        sender.send(Bytes::from(vec![4])).unwrap()
                                    }
                                    KeyCode::Char('z') => {
                                        sender.send(Bytes::from(vec![26])).unwrap()
                                    }
                                    KeyCode::Char('l') => {
                                        sender.send(Bytes::from(vec![12])).unwrap()
                                    }
                                    _ => {}
                                }
                            } else {
                                match key.code {
                                    KeyCode::Char('q') => return Ok(()),
                                    KeyCode::Char(input) => sender
                                        .send(Bytes::from(input.to_string().into_bytes()))
                                        .unwrap(),
                                    KeyCode::Backspace => {
                                        sender.send(Bytes::from(vec![8])).unwrap();
                                    }
                                    KeyCode::Enter => {
                                        #[cfg(unix)]
                                        sender.send(Bytes::from(vec![b'\n'])).unwrap();
                                        #[cfg(windows)]
                                        sender.send(Bytes::from(vec![b'\r', b'\n'])).unwrap();
                                    }
                                    KeyCode::Left => {
                                        sender.send(Bytes::from(vec![27, 91, 68])).unwrap()
                                    }
                                    KeyCode::Right => {
                                        sender.send(Bytes::from(vec![27, 91, 67])).unwrap()
                                    }
                                    KeyCode::Up => {
                                        sender.send(Bytes::from(vec![27, 91, 65])).unwrap()
                                    }
                                    KeyCode::Down => {
                                        sender.send(Bytes::from(vec![27, 91, 66])).unwrap()
                                    }
                                    KeyCode::Home => {
                                        sender.send(Bytes::from(vec![27, 91, 72])).unwrap()
                                    }
                                    KeyCode::End => {
                                        sender.send(Bytes::from(vec![27, 91, 70])).unwrap()
                                    }
                                    KeyCode::PageUp => {
                                        sender.send(Bytes::from(vec![27, 91, 53, 126])).unwrap()
                                    }
                                    KeyCode::PageDown => {
                                        sender.send(Bytes::from(vec![27, 91, 54, 126])).unwrap()
                                    }
                                    KeyCode::Tab => sender.send(Bytes::from(vec![9])).unwrap(),
                                    KeyCode::BackTab => {
                                        sender.send(Bytes::from(vec![27, 91, 90])).unwrap()
                                    }
                                    KeyCode::Delete => {
                                        sender.send(Bytes::from(vec![27, 91, 51, 126])).unwrap()
                                    }
                                    KeyCode::Insert => {
                                        sender.send(Bytes::from(vec![27, 91, 50, 126])).unwrap()
                                    }
                                    KeyCode::F(_) => todo!(),
                                    KeyCode::Null => todo!(),
                                    KeyCode::Esc => {
                                        sender.send(Bytes::from(vec![27])).unwrap();
                                    }
                                    KeyCode::CapsLock => todo!(),
                                    KeyCode::ScrollLock => todo!(),
                                    KeyCode::NumLock => todo!(),
                                    KeyCode::PrintScreen => todo!(),
                                    KeyCode::Pause => todo!(),
                                    KeyCode::Menu => todo!(),
                                    KeyCode::KeypadBegin => todo!(),
                                    KeyCode::Media(_) => todo!(),
                                    _ => {}
                                }
                            }
                        }
                    }
                    Event::FocusGained => {}
                    Event::FocusLost => {}
                    Event::Mouse(_) => {}
                    Event::Paste(_) => todo!(),
                    Event::Resize(cols, rows) => {
                        parser.write().unwrap().set_size(rows - 5, cols);
                        #[cfg(unix)]
                        {
                            if let Ok(master_lock) = master.lock() {
                                let raw_fd = master_lock.as_raw_fd();
                                unsafe {
                                    let mut winsize: libc::winsize = std::mem::zeroed();
                                    winsize.ws_row = rows - 5;
                                    winsize.ws_col = cols;
                                    let _ =
                                        libc::ioctl(raw_fd.unwrap(), libc::TIOCSWINSZ, &winsize);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
