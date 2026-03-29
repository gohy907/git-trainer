use crate::Frame;
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
}

struct PopupConfig {
    title: Option<String>,
    lines: Vec<Line<'static>>,
    color: Color,
    width: u16,
    height: u16,
}

impl Popup {
    fn config(&self, frame: &mut Frame) -> PopupConfig {
        match self {
            Popup::RunConifrmation => PopupConfig {
                title: Some("Подтвердите выбор".to_string()),
                lines: vec![
                    Line::from("Начать выполнение задания?").fg(Color::LightBlue),
                    Line::from("Enter — подтвердить, Esc — отменить").fg(Color::LightBlue),
                    Line::from(""),
                    Line::from(
                        "Внутри вы можете посмотреть условие задания командой git-trainer task",
                    )
                    .fg(Color::LightBlue),
                    Line::from("А сдать задание можно с помощью git-trainer submit")
                        .fg(Color::LightBlue),
                ],
                color: Color::LightBlue,
                width: std::cmp::max(frame.area().width / 3, 69),
                height: std::cmp::max(frame.area().height / 3, 7),
            },

            Popup::ResetConfirmation => PopupConfig {
                title: Some("Подтвердите перезагрузку".to_string()),
                lines: vec![
                    Line::from("Перезагрузить задание?").fg(Color::LightBlue),
                    Line::from("Вы потеряете все свои изменения.").fg(Color::LightBlue),
                    Line::from("Enter — подтвердить, Esc — отменить").fg(Color::LightBlue),
                ],
                color: Color::LightBlue,
                width: std::cmp::max(frame.area().width / 3, 35),
                height: std::cmp::max(frame.area().height / 3, 6),
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
                width: frame.area().width / 3,
                height: frame.area().height / 3,
            },
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let config = self.config(frame);
        let lines_of_popup = config.lines;

        let mut popup_block = Block::bordered()
            .fg(config.color)
            .title_alignment(Alignment::Center);

        if let Some(title) = config.title {
            popup_block = popup_block.title(title)
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
