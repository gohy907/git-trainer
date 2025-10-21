use crate::Frame;
use crate::app::App;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::Color;
use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph, Wrap},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let title = Line::from("git-trainer v0.0.1".bold());
    let mut lines_of_tasks = Vec::new();
    let mut active_description = Line::from("aboba".to_owned());
    for (i, (task, description)) in app.tasks.iter().enumerate() {
        let line: Line;
        if app.task_under_cursor == i {
            line = Line::from(task.clone())
                .left_aligned()
                .style(Style::default().bg(Color::White))
                .fg(Color::Black);
            active_description = Line::from(description.to_owned()).left_aligned();
        } else {
            line = Line::from(task.clone()).left_aligned();
        }

        lines_of_tasks.push(line);
    }

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

    let description_paragraph = Paragraph::new(active_description)
        .centered()
        .block(Block::bordered())
        .wrap(Wrap { trim: true });
    frame.render_widget(title, outer_layout[0]);
    frame.render_widget(tasks_paragraph, inner_layout[0]);
    frame.render_widget(description_paragraph, inner_layout[1]);
}
