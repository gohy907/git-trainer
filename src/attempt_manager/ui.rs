use crate::app::{App, VERSION};
use crate::test::{Test, TestStatus};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
    Block, Borders, Cell, List, ListItem, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table,
};

// Предварительно обработай текст: разбей на строки нужной ширины
fn a_wrap_text(text: &str, width: usize) -> Vec<String> {
    textwrap::wrap(text, width)
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

fn render_table(frame: &mut Frame, rect: Rect) {
    // Создай строку с многострочными ячейками
    let long_text = "Очень длинный текст, который нужно обрезать в несколько строк";
    let wrapped_lines = wrap_text(long_text, 50); // ширина 20 символов
    let height = wrapped_lines.len() as u16;

    // Собери текст обратно с \n
    let cell_text = wrapped_lines.join("\n");

    let row = Row::new(vec![
        Cell::from("ID"),
        Cell::from(cell_text),
        Cell::from("Status"),
    ])
    .height(height); // ⭐ УКАЗЫВАЕМ ВЫСОТУ СТРОКИ
    let widths = [
        Constraint::Length(10),     // колонка 1: фиксированная ширина 10 символов
        Constraint::Percentage(50), // колонка 2: 50% от доступного места
        Constraint::Min(20),        // колонка 3: минимум 20, остальное — сколько есть
    ];

    let table = Table::new(vec![row], widths).block(Block::default().borders(Borders::ALL));

    frame.render_widget(table, rect);
}

impl App {
    pub fn render_attempt_manager(&mut self, frame: &mut Frame) {
        let margin = ratatui::layout::Margin {
            horizontal: 1,
            vertical: 1,
        };

        let bordered_block = Block::default().borders(Borders::ALL);
        let global_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(frame.area());
        let title_area = global_area[0];
        let name_and_grade_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(global_area[1]);
        let main_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(global_area[2]);
        let explanation_area = global_area[3];

        let name_area = name_and_grade_area[0];
        let grade_area = name_and_grade_area[1];

        let attempts_area = main_area[0];
        let tests_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(3)])
            .split(main_area[1]);

        // let table_of_attempts_title_area = Layout::default()
        //     .direction(Direction::Horizontal)
        //     .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        //     .split(attempts_area[0]);
        // let table_of_attempts_area = attempts_area[1];

        let tests_title_area = tests_area[0];
        let tests_table_area = tests_area[1];

        let title = Line::from(format!("git-trainer v{}", VERSION))
            .centered()
            .bold();

        frame.render_widget(title, title_area);
        // frame.render_widget(&bordered_block, global_area[1]);
        // frame.render_widget(&bordered_block, global_area[2]);
        // // frame.render_widget(&bordered_block, table_of_attempts_area);
        // frame.render_widget(&bordered_block, attempts_area[0]);
        // let name = Paragraph::new("Current Task").alignment(Alignment::Left);
        // frame.render_widget(name, name_area);
        //
        // // Grade справа
        // let grade = Paragraph::new("Grade: 85/100").alignment(Alignment::Right);
        // frame.render_widget(grade, grade_area);
        let task_name = "Introduction to Git";
        let grade = format!("Grade: {}/100", 85);

        // Вычисляем ширину
        let area = global_area[1];
        let inner_width = area.width.saturating_sub(2); // минус рамки
        let text_length = (task_name.len() + grade.len()) as u16;
        let padding = inner_width.saturating_sub(text_length).max(1);

        // Создаём контент
        let content = Line::from(vec![
            Span::from(task_name),
            Span::raw(" ".repeat(padding as usize)),
            Span::from(grade),
        ]);

        let block = Block::default().borders(Borders::ALL);

        let paragraph = Paragraph::new(content).block(block);
        frame.render_widget(paragraph, area);
        render_attempts_table(frame, self, attempts_area);
        render_tests_table(frame, self, main_area[1]);
    }
}

// impl App {
//     pub fn load_tests(&mut self) {
//         self.tests = Some(vec![
//             Test {
//                 id: 1,
//                 description: "Убедитесь, что коммит в репозитории ровно один".to_string(),
//                 status: TestStatus::Failed,
//             },
//             Test {
//                 id: 2,
//                 description: "Код компилируется".to_string(),
//                 status: TestStatus::Passed,
//             },
//             Test {
//                 id: 3,
//                 description: "Убедитесь, что на экран выводится строка \"Hello, world!\"".to_string(),
//                 status: TestStatus::Failed,
//             },
//             Test {
//                 id: 4,
//                 description: "Программа должна корректно обрабатывать входные данные и выдавать правильный результат на различных тестовых случаях".to_string(),
//                 status: TestStatus::Passed,
//             },
//         ]);
//     }
// }

pub fn render_attempts_table(frame: &mut Frame, app: &mut App, area: Rect) {
    let attempts = app.submitted_attempts();
    let rows: Vec<Row> = attempts
        .iter()
        .map(|attempt| {
            Row::new(vec![
                attempt.timestamp.to_string(),
                attempt.grade.to_string(),
            ])
        })
        .collect();

    let header = Row::new(vec!["Дата попытки", "Оценка"])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let widths = [Constraint::Percentage(70), Constraint::Percentage(30)];

    // Создаём таблицу
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // Создаём scrollbar
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    // Область для scrollbar (правый край таблицы)
    let scrollbar_area = Rect {
        x: area.x + area.width - 1, // правый край
        y: area.y + 1,              // отступ сверху для рамки
        width: 1,
        height: area.height.saturating_sub(2), // -2 для верхней и нижней рамки
    };

    app.attempts_table_config.attempts_scrollbar_state = app
        .attempts_table_config
        .attempts_scrollbar_state
        .content_length(attempts.len())
        .position(
            app.attempts_table_config
                .attempts_table_state
                .selected()
                .unwrap_or(0),
        );

    frame.render_stateful_widget(
        table,
        area,
        &mut app.attempts_table_config.attempts_table_state,
    );
    frame.render_stateful_widget(
        scrollbar,
        scrollbar_area,
        &mut app.attempts_table_config.attempts_scrollbar_state,
    );
}
pub fn render_tests_table(frame: &mut Frame, app: &mut App, area: Rect) {
    let tests = &app.submitted_attempts()[app.attempts_table_config.attempt_under_cursor].tests;
    let passed_count = tests.iter().filter(|t| t.passed == true).count();
    let total_count = tests.len();

    let title = if passed_count == total_count {
        format!("Тесты: пройдены все ({}/{})", passed_count, total_count)
    } else {
        format!("Тесты: пройдены не все ({}/{})", passed_count, total_count)
    };

    let items: Vec<ListItem> = tests
        .iter()
        .map(|test| {
            let lines = wrap_text(&test.description, area.width.saturating_sub(6) as usize);

            let text_lines = vec![Line::from(vec![
                Span::styled(
                    format!("{}. ", test.id),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(lines[0].clone(), Style::default()),
            ])];

            // for line in lines.iter() {
            //     text_lines.push(Line::from(vec![Span::styled(
            //         line.clone(),
            //         Style::default(),
            //     )]));
            // }

            ListItem::new(Text::from(text_lines))
        })
        .collect();

    // Создаём список
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::White)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // Scrollbar
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("▲"))
        .end_symbol(Some("▼"))
        .track_symbol(Some("┃"))
        .thumb_symbol("█")
        .style(Style::default().fg(Color::DarkGray))
        .thumb_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    let scrollbar_area = Rect {
        x: area.x + area.width - 1,
        y: area.y + 1,
        width: 1,
        height: area.height.saturating_sub(2),
    };

    // Обновляем состояние scrollbar
    app.tests_scrollbar_state = app
        .tests_scrollbar_state
        .content_length(tests.len())
        .position(app.tests_list_state.selected().unwrap_or(0));
    frame.render_stateful_widget(list, area, &mut app.tests_list_state);

    frame.render_stateful_widget(scrollbar, scrollbar_area, &mut app.tests_scrollbar_state);
}

// Функция для переноса длинного текста
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width {
            if !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
            }

            if word.len() > max_width {
                for chunk in word.as_bytes().chunks(max_width) {
                    lines.push(String::from_utf8_lossy(chunk).to_string());
                }
            } else {
                current_line = word.to_string();
            }
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
