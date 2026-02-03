use crate::app::{App, VERSION};
use crate::db::{TaskStatus, TestResult, format_timestamp};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
    Block, Borders, List, ListItem, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table,
};

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
        let task_name = self.task_under_cursor().name.clone();
        let task_status = &self.task_under_cursor().status;

        let span_status_style = match task_status {
            TaskStatus::NotInProgress => Span::from(task_status.to_string()).fg(Color::Red),
            TaskStatus::InProgress => Span::from(task_status.to_string().fg(Color::Yellow)),
            TaskStatus::Done => Span::from(task_status.to_string()).fg(Color::Blue),
            TaskStatus::Approved => Span::from(task_status.to_string()).fg(Color::LightGreen),
            _ => Span::from(task_status.to_string()).fg(Color::DarkGray),
        };

        let area = global_area[1];
        let inner_width = area.width.saturating_sub(2);
        let text_length =
            (task_name.chars().count() + task_status.to_string().chars().count()) as u16;
        let padding = inner_width.saturating_sub(text_length).max(1);

        let content = Line::from(vec![
            Span::from(task_name),
            Span::raw(" ".repeat(padding as usize)),
            span_status_style,
        ]);

        let block = Block::default().borders(Borders::ALL);

        let paragraph = Paragraph::new(content).block(block);
        frame.render_widget(paragraph, area);
        render_attempts_table(frame, self, attempts_area);
        render_tests_table(frame, self, main_area[1]);
    }
}

pub fn render_attempts_table(frame: &mut Frame, app: &mut App, area: Rect) {
    let attempts = app.attempts_of_choosed_task();

    let mut rows = Vec::new();
    for attempt in attempts {
        let tests = app
            .repo
            .get_attempt_tests(attempt.id)
            .expect("While working with db:");
        let passed_count = tests
            .iter()
            .filter(|t| t.result == TestResult::Passed)
            .count();
        let total_count = tests.len();
        let tests_passed = format!("{}/{}", passed_count, total_count);
        let row = Row::new(vec![
            attempt
                .timestamp
                .as_ref()
                .expect("While working with db:")
                .clone(),
            tests_passed,
        ]);
        rows.push(row);
    }

    let header = Row::new(vec!["Дата попытки", "Тесты"])
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
    let tests = app.tests_of_choosed_attempt();
    let passed_count = tests
        .iter()
        .filter(|t| t.result == TestResult::Passed)
        .count();
    let total_count = tests.len();

    let title = if passed_count == total_count {
        format!("Тесты: пройдены все ({}/{})", passed_count, total_count)
    } else {
        format!("Тесты: пройдены не все ({}/{})", passed_count, total_count)
    };

    let items: Vec<ListItem> = tests
        .iter()
        .map(|test| {
            let style = match test.result {
                TestResult::Passed => Style::default().fg(Color::LightGreen),
                TestResult::Failed => Style::default().fg(Color::Red),
                TestResult::NotExecuted => Style::default().fg(Color::DarkGray),
            };
            let width = area.width;
            let mut lines: Vec<String> = textwrap::wrap(&test.description, width as usize)
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            lines.pop();
            let lines = wrap_text(&test.description, width.into());

            let mut text_lines = Vec::new();

            for line in lines.iter() {
                text_lines.push(Line::from(vec![Span::styled(line.clone(), style)]));
            }

            ListItem::new(Text::from(text_lines))
        })
        .collect();

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

    app.tests_scrollbar_state = app
        .tests_scrollbar_state
        .content_length(tests.len())
        .position(app.tests_list_state.selected().unwrap_or(0));
    frame.render_stateful_widget(list, area, &mut app.tests_list_state);

    frame.render_stateful_widget(scrollbar, scrollbar_area, &mut app.tests_scrollbar_state);
}

// Функция для переноса длинного текста
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines: Vec<String> = textwrap::wrap(text, max_width)
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    lines.pop();
    lines
}
