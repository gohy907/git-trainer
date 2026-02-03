use crate::app::App;
use crate::app::VERSION;
use crate::db::TaskStatus;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize, palette::tailwind};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Cell, Paragraph, Row, Table};

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
    for task in app.repo.get_all_tasks().expect("While working with db:") {
        if task.name.chars().count() > max {
            max = task.name.chars().count();
        }
    }
    max
}

impl App {
    pub fn render_main_menu(&mut self, frame: &mut Frame) {
        let title = Line::from(format!("git-trainer v{}", VERSION).bold()).centered();
        // attempt_manager::ui::ui(frame, app);

        let global_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(frame.area());

        let how_to_use_string =
            "← ↑ ↓ → — перемещение, q — выход, Enter — начать задание, r — перезагрузить задание, Пробел — открыть менеджер попыток"
                .to_string();
        let how_to_use = Paragraph::new(how_to_use_string).centered();

        frame.render_widget(how_to_use, global_layout[2]);
        frame.render_widget(title, global_layout[0]);

        render_table(frame, global_layout[1], self);
        if let Some(popup) = &self.active_popup {
            popup.render(frame, self);
        }
    }
}

fn render_table(frame: &mut Frame, rect: Rect, app: &mut App) {
    let max_task_name_length = get_max_task_name_length(app) as u16;
    let colors = TableColors::new();

    let header = ["Название", "Описание", "Статус"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);
    let binding = app.repo.get_all_tasks().expect("While working with db: ");
    let rows = binding.iter().enumerate().map(|(i, data)| {
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

        let wrapped_desc = wrap(&data.description, LINE_WIDTH as usize, cell_height);
        let item = [
            data.name.clone(),
            wrapped_desc,
            // "a\nb".to_string(),
            status_str.to_string(),
            // match data.grade {
            //     Some(grade) => format!("{}/100", grade.to_string()),
            //     None => "Нет оценки".to_string(),
            // },
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
            Constraint::Length(max_task_name_length + 6),
            Constraint::Min(LINE_WIDTH),
            Constraint::Min(10),
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
