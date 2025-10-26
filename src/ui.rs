use crate::Frame;
use crate::app::App;
use ratatui::layout::{Alignment, Flex};
use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Clear, Paragraph, Wrap},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let title = Line::from("git-trainer v0.0.1".bold()).centered();
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
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[1]);

    let tasks_paragraph = Paragraph::new(lines_of_tasks)
        .centered()
        .block(Block::bordered().title("Задания"));

    let description_paragraph = Paragraph::new(active_description)
        .centered()
        .block(Block::bordered().title("Описание"))
        .wrap(Wrap { trim: true });

    let how_to_use_string = "← ↑ ↓ → для перемещения, q для выхода".to_string();
    let how_to_use = Paragraph::new(how_to_use_string).centered();
    frame.render_widget(how_to_use, outer_layout[2]);
    frame.render_widget(title, outer_layout[0]);
    frame.render_widget(tasks_paragraph, inner_layout[0]);
    frame.render_widget(description_paragraph, inner_layout[1]);

    if app.is_popup_active {
        let lines_of_popup = vec![
            popup_line("Начать выполнение задания?"),
            popup_line("Enter — подтвердить, Esc — отменить"),
        ];

        let popup_block = Block::bordered()
            .fg(Color::LightBlue)
            .title("Подтвердите выбор")
            .title_alignment(Alignment::Center);

        let popup_content = Paragraph::new(lines_of_popup)
            .centered()
            .style(Style::default().fg(Color::LightBlue))
            .wrap(Wrap { trim: true });

        let area = popup_area(frame.area(), 40, 10);

        let aboba = popup_area(area, 40, 2);

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(popup_content, aboba);
    }
}

fn popup_area(area: Rect, x: u16, y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn popup_line<'a>(s: &'a str) -> Line<'a> {
    Line::from(s).style(Style::default().fg(Color::LightBlue))
}
