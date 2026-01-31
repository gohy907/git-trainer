use crate::App;
use crate::Frame;
use crate::db::Task;
use ratatui::layout::Constraint;
use ratatui::layout::{Alignment, Flex};
use ratatui::prelude::{Layout, Rect};
use ratatui::style::Color;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::{
    text::Line,
    widgets::{Clear, Wrap},
};

#[derive(Clone)]
pub enum Popup {
    RunConifrmation,
    ResetConfirmation,
    ResetDone,
    Error(String),
    Help,
}

struct PopupConfig {
    title: Option<String>,
    lines: Vec<Line<'static>>,
    color: Color,
    width: u16,
    height: u16,
}

impl Popup {
    fn config(&self, app: &App, frame: &mut Frame) -> PopupConfig {
        match self {
            Popup::RunConifrmation => PopupConfig {
                title: Some("Подтвердите выбор".to_string()),
                lines: vec![
                    Line::from("Начать выполнение задания?").fg(Color::LightBlue),
                    Line::from("Enter — подтвердить, Esc — отменить").fg(Color::LightBlue),
                ],
                color: Color::LightBlue,
                width: frame.area().width * 1 / 3,
                height: frame.area().height * 1 / 3,
            },

            Popup::ResetConfirmation => PopupConfig {
                title: Some("Подтвердите перезагрузку".to_string()),
                lines: vec![
                    Line::from("Перезагрузить задание?").fg(Color::LightBlue),
                    Line::from("Вы потеряете все свои изменения.").fg(Color::LightBlue),
                    Line::from("Enter — подтвердить, Esc — отменить").fg(Color::LightBlue),
                ],
                color: Color::LightBlue,
                width: frame.area().width * 1 / 3,
                height: frame.area().height * 1 / 3,
            },

            Popup::Error(error) => PopupConfig {
                title: Some("Ошибка!".to_string()),
                lines: vec![
                    Line::from(error.clone()).fg(Color::Red),
                    Line::from(""),
                    Line::from("Обратитесь к преподавателю.").fg(Color::Red),
                ],
                color: Color::Red,
                width: frame.area().width * 2 / 3,
                height: frame.area().height * 2 / 3,
            },

            Popup::ResetDone => PopupConfig {
                title: None,
                lines: vec![
                    Line::from("Задание перезагружено.").fg(Color::LightGreen),
                    Line::from("Нажмите Enter, чтобы продолжить").fg(Color::LightGreen),
                ],
                color: Color::LightGreen,
                width: frame.area().width * 1 / 3,
                height: frame.area().height * 1 / 3,
            },

            Popup::Help => {
                // let desc = &app.config.tasks[app.task_under_cursor].extended_desc;

                let mut lines = Vec::new();

                for line in app.task_choosed().extended_description().lines() {
                    lines.push(Line::from(line.to_string()).fg(Color::LightBlue));
                }
                lines.push(Line::from(""));
                lines.push(Line::from("Нажмите Enter, чтобы продолжить").fg(Color::LightBlue));
                PopupConfig {
                    title: None,
                    lines: lines,
                    color: Color::LightBlue,
                    width: frame.area().width * 2 / 3,
                    height: frame.area().height * 2 / 3,
                }
            }
        }
    }

    pub fn render(&self, frame: &mut Frame, app: &App) {
        let config = self.config(app, frame);
        let lines_of_popup = config.lines;

        let mut popup_block = Block::bordered()
            .fg(config.color)
            .title_alignment(Alignment::Center);

        match config.title {
            Some(title) => popup_block = popup_block.title(title),
            None => {}
        }

        let popup_content = Paragraph::new(lines_of_popup)
            .centered()
            .style(Style::default().fg(config.color))
            .wrap(Wrap { trim: true });

        let area = popup_area(frame.area(), config.width, config.height);

        let aboba = popup_area(area, config.width - 2, config.height / 2);

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
