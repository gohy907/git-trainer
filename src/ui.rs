use crate::Frame;
use crate::app::App;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let title = Line::from("git-trainer v0.0.1-beta-alpha-gamma-shtrih".bold());
    let mut lines_of_tasks = Vec::new();
    for task in app.names_of_tasks.clone() {
        lines_of_tasks.push(Line::from(task).left_aligned())
    }
    // let a = Line::from(self.names_of_tasks);
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[1]);

    let tasks_paragraph = Paragraph::new(lines_of_tasks)
        .centered()
        .block(Block::bordered());

    frame.render_widget(title, outer_layout[0]);
    frame.render_widget(tasks_paragraph, inner_layout[0]);
}
