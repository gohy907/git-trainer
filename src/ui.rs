use crate::App;
use crate::Frame;
use crate::TaskStatus;
use ratatui::layout::Constraint;
use ratatui::layout::{Alignment, Flex};
use ratatui::prelude::{Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::style::palette::tailwind;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::{Block, Cell, Row, Table};
use ratatui::{
    text::{Line, Text},
    widgets::{Clear, Wrap},
};
use tui_term::vt100;
use tui_term::widget::PseudoTerminal;
use vt100::Screen;

const LINE_WIDTH: u16 = 50;
const VERSION: &str = "0.0.3";

fn wrap(text: &str, width: usize, max_lines: usize) -> String {
    if width == 0 || max_lines == 0 {
        return String::new();
    }

    let mut lines = Vec::new();
    let mut current = String::new();
    let mut current_len = 0;

    for word in text.split_whitespace() {
        let word_len = word.chars().count();

        if word_len > width {
            if !current.is_empty() {
                lines.push(current);
                current = String::new();
                current_len = 0;
                if lines.len() == max_lines {
                    break;
                }
            }

            let mut buf = String::new();
            let mut len = 0usize;
            for ch in word.chars() {
                if len == width {
                    lines.push(buf);
                    buf = String::new();
                    len = 0;
                    if lines.len() == max_lines {
                        break;
                    }
                }
                buf.push(ch);
                len += 1;
            }
            if lines.len() == max_lines {
                break;
            }
            if !buf.is_empty() {
                current = buf;
                current_len = len;
            }
            continue;
        }

        let extra = if current.is_empty() {
            word_len
        } else {
            1 + word_len
        };
        if current_len + extra > width {
            if !current.is_empty() {
                lines.push(current);
                if lines.len() == max_lines {
                    return lines.join("\n");
                }
            }
            current = word.to_string();
            current_len = word_len;
        } else {
            if !current.is_empty() {
                current.push(' ');
                current_len += 1;
            }
            current.push_str(word);
            current_len += word_len;
        }
    }

    if !current.is_empty() && lines.len() < max_lines {
        lines.push(current);
    }

    lines.join("\n")
}

pub struct TableColors {
    normal_row_color: Color,
    alt_row_color: Color,
    selected_row_style_fg: Color,
}

impl TableColors {
    pub fn new() -> TableColors {
        TableColors {
            selected_row_style_fg: tailwind::BLUE.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
        }
    }
}

fn get_max_task_name_length(app: &App) -> usize {
    let mut max = usize::min_value();
    for task in &app.config.tasks {
        if task.name.chars().count() > max {
            max = task.name.chars().count();
        }
    }
    max
}

fn render_table(frame: &mut Frame, rect: Rect, app: &mut App) {
    let max_task_name_length = get_max_task_name_length(app) as u16;
    let colors = TableColors::new();

    let header = ["Название", "Описание", "Статус", "Оценка"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);

    let rows = app.config.tasks.iter().enumerate().map(|(i, data)| {
        let row_bg = match i % 2 {
            0 => colors.normal_row_color,
            _ => colors.alt_row_color,
        };

        let status_str = match data.status {
            TaskStatus::NotInProgress => "НЕ НАЧАТО",
            TaskStatus::InProgress => "НАЧАТО",
            TaskStatus::Done => "СДЕЛАНО",
            TaskStatus::Pending => "ОТПРАВЛЕНО",
            TaskStatus::Approved => "ОЦЕНЕНО",
        };

        let cell_height = 4;

        let wrapped_desc = wrap(&data.desc, LINE_WIDTH as usize, cell_height);
        let item = [
            data.name.clone(),
            wrapped_desc,
            status_str.to_string(),
            match data.grade {
                Some(grade) => format!("{}/100", grade.to_string()),
                None => "Нет оценки".to_string(),
            },
        ];

        let cells = item.into_iter().enumerate().map(|(col, content)| {
            let mut cell =
                Cell::from(Text::from(content)).style(Style::new().fg(Color::White).bg(row_bg));

            if col == 2 {
                let status_color = match data.status {
                    TaskStatus::NotInProgress => Color::Red,
                    TaskStatus::InProgress => Color::Yellow,
                    TaskStatus::Done => Color::Blue,
                    TaskStatus::Pending => Color::LightMagenta,
                    TaskStatus::Approved => Color::LightGreen,
                };

                cell = cell.style(Style::new().fg(status_color).bg(row_bg));
            }

            cell
        });

        Row::new(cells).height(4).style(Style::new().bg(row_bg))
    });

    let bar = ">>";
    let selected_row_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(colors.selected_row_style_fg);

    let t = Table::new(
        rows,
        [
            Constraint::Min(max_task_name_length),
            Constraint::Min(LINE_WIDTH),
            Constraint::Min(10),
            Constraint::Min(14),
        ],
    )
    .header(header)
    .row_highlight_style(selected_row_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]));

    frame.render_stateful_widget(t, rect, &mut app.table_state);
}

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
                let desc = &app.config.tasks[app.task_under_cursor].extended_desc;

                let mut lines = Vec::new();

                for line in desc {
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

pub fn ui(frame: &mut Frame, app: &mut App) {
    let title = Line::from(format!("git-trainer v{}", VERSION).bold()).centered();

    let global_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let how_to_use_string =
        "← ↑ ↓ → — перемещение, q — выход, Enter — начать задание, r — перезагрузить задание"
            .to_string();
    let how_to_use = Paragraph::new(how_to_use_string).centered();

    frame.render_widget(how_to_use, global_layout[2]);
    frame.render_widget(title, global_layout[0]);

    render_table(frame, global_layout[1], app);
    if let Some(popup) = &app.active_popup {
        popup.render(frame, app);
    }
}

fn popup_area(area: Rect, x: u16, y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub fn ui_pty(f: &mut Frame, screen: &Screen, app: &mut App) {
    let title = Line::from(format!("git-trainer v{}", VERSION).bold()).centered();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.area());
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().add_modifier(Modifier::BOLD));
    let pseudo_term = PseudoTerminal::new(screen).block(block);
    let explanation = "Напишите команду exit для выхода".to_string();
    let explanation = Paragraph::new(explanation)
        .style(Style::default())
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);
    f.render_widget(pseudo_term, chunks[1]);
    f.render_widget(explanation, chunks[2]);

    if let Some(popup) = &app.active_popup {
        popup.render(f, app);
    }
}
