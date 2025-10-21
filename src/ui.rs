use crate::app::App;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" git-trainer v0.0.1 ".bold());
        let mut b = Vec::new();
        for task in self.names_of_tasks.clone() {
            b.push(Line::from(task).left_aligned())
        }
        // let a = Line::from(self.names_of_tasks);
        let block = Block::bordered().title(title.centered());

        Paragraph::new(b).centered().block(block).render(area, buf);
    }
}
