use crate::Frame;
use crate::app::{App, Status};
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

    let active_description = app
        .config
        .tasks
        .iter()
        .enumerate()
        .find_map(|(i, task)| {
            (app.task_under_cursor == i).then(|| Line::from(task.desc.clone()).left_aligned())
        })
        .expect("Active task must exist");

    let mut lines_of_tasks = Vec::new();

    for (i, task) in app.config.tasks.iter().enumerate() {
        let line = if app.task_under_cursor == i {
            Line::from(task.name.clone())
                .left_aligned()
                .style(Style::default().bg(Color::White))
                .fg(Color::Black)
        } else {
            Line::from(task.name.clone()).left_aligned()
        };

        lines_of_tasks.push(line);
    }

    let mut lines_of_statuses = Vec::new();
    for task in &app.config.tasks {
        let status = task.status;
        let status_str = match status {
            Status::NotInProgress => "НЕ НАЧАТО",
            Status::InProgress => "НАЧАТО",
            Status::Done => "СДЕЛАНО",
            Status::Pending => "ОТПРАВЛЕНО",
            Status::Approved => "ОЦЕНЕНО",
        };
        let line = Line::from(status_str);
        lines_of_statuses.push(line);
    }

    let global_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(global_layout[1]);

    let task_list_layout = main_layout[0];

    let [list, status] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(task_list_layout);

    let tasks_paragraph = Paragraph::new(lines_of_tasks)
        .centered()
        .block(Block::bordered().title("Задания"));
    let statuses_paragraph = Paragraph::new(lines_of_statuses)
        .centered()
        .block(Block::bordered().title("Состояния"));

    let description_paragraph = Paragraph::new(active_description)
        .centered()
        .block(Block::bordered().title("Описание"))
        .wrap(Wrap { trim: true });

    let how_to_use_string = "← ↑ ↓ → для перемещения, q для выхода".to_string();
    let how_to_use = Paragraph::new(how_to_use_string).centered();

    frame.render_widget(how_to_use, global_layout[2]);
    frame.render_widget(title, global_layout[0]);
    frame.render_widget(tasks_paragraph, list);
    frame.render_widget(statuses_paragraph, status);
    frame.render_widget(description_paragraph, main_layout[1]);

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
