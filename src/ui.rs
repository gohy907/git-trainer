use crate::Frame;
use crate::app::{App, Status};
use ratatui::layout::Constraint;
use ratatui::layout::{Alignment, Flex};
use ratatui::prelude::{Direction, Layout, Rect};
use ratatui::style::Color;
use ratatui::style::palette::tailwind;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Cell, Row, Table};
use ratatui::{
    text::{Line, Text},
    widgets::{Clear, Paragraph, Wrap},
};

pub struct TableColors {
    pub normal_row_color: Color,
    pub alt_row_color: Color,
}

impl TableColors {
    pub fn new() -> TableColors {
        TableColors {
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
        }
    }
}

fn get_max_task_name_length(app: &App) -> usize {
    let mut max = usize::min_value();
    for task in &app.config.tasks {
        if task.name.len() > max {
            max = task.name.len();
        }
    }
    max
}

fn get_max_task_desc_length(app: &App) -> usize {
    let mut max = usize::min_value();
    for task in &app.config.tasks {
        if task.desc.len() > max {
            max = task.desc.len();
        }
    }
    usize::min(57, max)
}

fn render_table(frame: &mut Frame, rect: Rect, app: &App) {
    // Чтобы красить каждую клетку по отдельности надо их все иметь как набор Cell-ов
    // Но тогда не получится единый фон на все клетки в ряду поставить, они будут с пустым
    // промежутком
    // Так что здесь я два раза рендерю таблицу - сначала без текста, но с фоном,
    // а потом текст поверх фона
    // Лютейший костыль, но работает
    let max_task_name_length = get_max_task_name_length(app) as u16;
    let max_task_desc_length = get_max_task_desc_length(app) as u16;
    let colors = TableColors::new();

    let header = [""].into_iter().map(Cell::from).collect::<Row>().height(1);
    let rows_without_text = app.config.tasks.iter().enumerate().map(|(i, _)| {
        let color = match i % 2 {
            0 => colors.normal_row_color,
            _ => colors.alt_row_color,
        };

        [""].into_iter()
            .collect::<Row>()
            .style(Style::new().bg(color))
            .height(4)
    });

    let bar = ">>";
    let t = Table::new(
        rows_without_text,
        [
            Constraint::Length(max_task_name_length + 5),
            Constraint::Length(max_task_desc_length + 5),
            Constraint::Min(2),
        ],
    )
    .header(header)
    // .row_highlight_style(selected_row_style)
    // .column_highlight_style(selected_col_style)
    // .cell_highlight_style(selected_cell_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]));

    frame.render_widget(t, rect);

    let rows = app.config.tasks.iter().enumerate().map(|(_, data)| {
        let status_str = match data.status {
            Status::NotInProgress => "НЕ НАЧАТО",
            Status::InProgress => "НАЧАТО",
            Status::Done => "СДЕЛАНО",
            Status::Pending => "ОТПРАВЛЕНО",
            Status::Approved => "ОЦЕНЕНО",
        };

        let item = [data.name.clone(), data.desc.clone(), status_str.to_string()];

        let cells = item.into_iter().enumerate().map(|(col, content)| {
            let base = Cell::from(Text::from(content)).style(Style::new().fg(Color::White));

            if col == 2 {
                let status_color = match data.status {
                    Status::NotInProgress => Color::Red,
                    Status::InProgress => Color::Yellow,
                    Status::Done => Color::Blue,
                    Status::Pending => Color::LightMagenta,
                    Status::Approved => Color::LightGreen,
                };

                base.style(Style::new().fg(status_color))
            } else {
                base
            }
        });

        Row::new(cells).height(4)
    });

    let header = ["Название", "Описание", "Статус"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);
    let bar = ">>";
    let t = Table::new(
        rows,
        [
            Constraint::Length(max_task_name_length + 5),
            Constraint::Length(max_task_desc_length + 5),
            Constraint::Min(2),
        ],
    )
    .header(header)
    // .row_highlight_style(selected_row_style)
    // .column_highlight_style(selected_col_style)
    // .cell_highlight_style(selected_cell_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]));

    frame.render_widget(t, rect);
}

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

    // .bg(self.colors.buffer_bg)
    // .highlight_spacing(HighlightSpacing::Always);
    // let table = Table::new(rows, widths)
    //     // ...and they can be separated by a fixed spacing.
    //     .column_spacing(1)
    //     // You can set the style of the entire Table.
    //     .style(Style::new().blue())
    //     // It has an optional header, which is simply a Row always visible at the top.
    //     .header(
    //         Row::new(vec!["Название", "Состояние", "Описание"])
    //             .style(Style::new().bold())
    //             // To add space between the header and the rest of the rows, specify the margin
    //             .bottom_margin(1),
    //     )
    //     // It has an optional footer, which is simply a Row always visible at the bottom.
    //     .footer(Row::new(vec!["Updated on Dec 28"]))
    //     // As any other widget, a Table can be wrapped in a Block.
    //     .block(Block::new().title("Задания"))
    //     // The selected row, column, cell and its content can also be styled.
    //     .row_highlight_style(Style::new().reversed())
    //     .column_highlight_style(Style::new().red())
    //     .cell_highlight_style(Style::new().blue())
    //     // ...and potentially show a symbol in front of the selection.
    //     .highlight_symbol(">>");
    //
    // let main_layout = Layout::default()
    //     .direction(Direction::Horizontal)
    //     // .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    //     .split(global_layout[1]);

    // let task_list_layout = main_layout[0];

    // let [list, status] =
    //     Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
    //         .areas(task_list_layout);

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

    // let header = ["Название", "Описание", "Статус"]
    //     .into_iter()
    //     .map(Cell::from)
    //     .collect::<Row>()
    //     .height(1);
    // let t = Table::new(
    //     rows_1,
    //     [
    //         // + 1 is for padding.
    //         Constraint::Min(10 + 1),
    //         Constraint::Min(1 + 1),
    //         Constraint::Min(2),
    //     ],
    // )
    // .header(header)
    // // .row_highlight_style(selected_row_style)
    // // .column_highlight_style(selected_col_style)
    // // .cell_highlight_style(selected_cell_style)
    // .highlight_symbol(Text::from(vec![
    //     "".into(),
    //     bar.into(),
    //     bar.into(),
    //     "".into(),
    // ]));
    //
    // frame.render_widget(statuses_paragraph, status);
    // frame.render_widget(description_paragraph, main_layout[1]);

    render_table(frame, global_layout[1], app);
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
