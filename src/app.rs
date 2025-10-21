use crate::DefaultTerminal;
use crate::Frame;
use crate::app::event::Event;
use crate::io;
use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
pub struct App {
    pub names_of_tasks: Vec<String>,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            names_of_tasks: vec!["Привет, мир!".to_string(), "Своих не сдаём".to_string()],
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}
