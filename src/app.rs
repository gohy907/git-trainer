use crate::Backend;
use crate::Terminal;
use crate::app::event::Event;
use crate::docker;
use crate::io;
use crate::ui;
use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;

pub struct Task {
    pub name: String,
    pub internal_name: String,
    pub description: String,
}

impl Task {
    pub fn new(name: &str, internal_name: &str, description: &str) -> Task {
        Task {
            name: name.to_string(),
            internal_name: internal_name.to_string(),
            description: description.to_string(),
        }
    }
}

// TODO: Rewrite tasks in struct
pub struct App {
    pub tasks: Vec<Task>,
    pub task_under_cursor: usize,
    pub is_popup_active: bool,
    pub exit: bool,
    pub task_to_run: Option<String>,
}

impl App {
    pub fn new() -> App {
        App {
            tasks: vec![
                Task::new(
                    "Привет, мир!",
                    "task-1",
                    "В этой задаче Вам предстоит создать новый Git репозиторий \
                    и сделать в нём первый коммит.",
                ),
                Task::new(
                    "Своих не сдаём!",
                    "task-2",
                    "Последний коммит в этой задаче посеял в коде критический баг. \
                     Вам нужно исправить этот баг, не создавая нового коммита.",
                ),
                Task::new(
                    "Ещё какое-то там задание",
                    "task-3",
                    "Надо двери в котельную замерить",
                ),
            ],
            exit: false,
            is_popup_active: false,
            task_under_cursor: 0,
            task_to_run: None,
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
            KeyCode::Enter => {
                if self.is_popup_active {
                    self.task_to_run =
                        Some(self.tasks[self.task_under_cursor].internal_name.clone());
                    self.exit();
                    self.is_popup_active = false;
                } else {
                    self.is_popup_active = true;
                }
            }

            KeyCode::Esc => {
                self.is_popup_active = false;
            }
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
