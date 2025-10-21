use crate::Backend;
use crate::Terminal;
use crate::app::event::Event;
use crate::io;
use crate::ui;
use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
pub struct App {
    pub tasks: Vec<(String, String)>,
    pub task_under_cursor: usize,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            tasks: vec![
                (
                    "Привет, мир!".to_string(),
                    "В этой задаче Вам предстоит создать новый Git репозиторий и сделать в нём первый коммит.".to_string(),
                ),
                (
                    "Своих не сдаём!".to_string(), 
                    "Последний коммит в этой задаче посеял в коде критический баг. Вам нужно исправить этот баг, не создавая нового коммита.".to_string(),
                ),
                (
                "Ещё какое-то там задание".to_string(),
                "Надо двери в котельную замерить".to_string()
            )
            ],
            exit: false,
            task_under_cursor: 0,
        }
    }

    pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|f| ui(f, &self))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Down => self.move_cursor_down(),
            _ => {}
        }
    }

    fn move_cursor_up(&mut self) {
        if self.task_under_cursor != 0 {
            self.task_under_cursor -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        if self.task_under_cursor != self.tasks.len() - 1 {
            self.task_under_cursor += 1;
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
