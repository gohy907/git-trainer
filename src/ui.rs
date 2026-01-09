use crate::Frame;
use crate::TaskStatus;
use crate::app;
use crate::app::App;
use ratatui::layout::Constraint;
use ratatui::layout::{Alignment, Flex};
use ratatui::prelude::{Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::style::palette::tailwind;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::{Block, Cell, Row, Table};
use ratatui::{
    text::{Line, Text},
    widgets::{Clear, Paragraph, Wrap},
};

const LINE_WIDTH: u16 = 50;

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
    Error(String),
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    let title = Line::from("git-trainer v0.0.2".bold()).centered();

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
    match app.active_popup {
        Some(ref popup) => {
            let (popup_block, popup_content, block_area, content_area) = match popup {
                Popup::RunConifrmation => {
                    let lines_of_popup = vec![
                        popup_line("Начать выполнение задания?", Color::LightBlue),
                        popup_line("Enter — подтвердить, Esc — отменить", Color::LightBlue),
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
                    (popup_block, popup_content, area, aboba)
                }
                Popup::ResetConfirmation => {
                    let lines_of_popup = vec![
                        popup_line("Перезагрузить задание?", Color::LightBlue),
                        popup_line("Вы потеряете все свои изменения.", Color::LightBlue),
                        popup_line("Enter — подтвердить, Esc — отменить", Color::LightBlue),
                    ];

                    let popup_block = Block::bordered()
                        .fg(Color::LightBlue)
                        .title("Подтвердите перезагрузку")
                        .title_alignment(Alignment::Center);

                    let popup_content = Paragraph::new(lines_of_popup)
                        .centered()
                        .style(Style::default().fg(Color::LightBlue))
                        .wrap(Wrap { trim: true });

                    let area = popup_area(frame.area(), 40, 10);

                    let aboba = popup_area(area, 40, 3);
                    (popup_block, popup_content, area, aboba)
                }
                Popup::Error(error) => {
                    let lines_of_popup = vec![
                        popup_line(&error, Color::Red),
                        popup_line("Обратитесь к преподавателю.", Color::Red),
                    ];

                    let popup_block = Block::bordered()
                        .fg(Color::Red)
                        .title("Ошибка!")
                        .title_alignment(Alignment::Center);

                    let popup_content = Paragraph::new(lines_of_popup)
                        .centered()
                        .style(Style::default().fg(Color::Red))
                        .wrap(Wrap { trim: true });

                    let area = popup_area(frame.area(), 60, 20);

                    let aboba = popup_area(area, 40, 3);

                    (popup_block, popup_content, area, aboba)
                }
            };

            frame.render_widget(Clear, block_area);
            frame.render_widget(popup_block, block_area);
            frame.render_widget(popup_content, content_area);
        }
        None => {}
    }
}

fn popup_area(area: Rect, x: u16, y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn popup_line<'a>(s: &'a str, color: Color) -> Line<'a> {
    Line::from(s).style(Style::default().fg(color))
}
