use crate::app::{App, AttemptManagerStatus, VERSION};
use crate::db::{TaskStatus, TestResult};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
    Block, Borders, List, ListItem, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table,
};
use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

impl App {
    pub fn render_attempt_manager(&mut self, frame: &mut Frame) {
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
        let main_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(global_area[2]);

        let attempts_area = main_area[0];

        let title = Line::from(format!("git-trainer v{}", VERSION))
            .centered()
            .bold();

        frame.render_widget(title, title_area);
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
    for (i, attempt) in attempts.iter().enumerate() {
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

        let style = if app.attempt_manager_config.status == AttemptManagerStatus::SelectingTests {
            if passed_count == total_count {
                Style::new().fg(Color::LightGreen)
            } else {
                Style::new().fg(Color::Red)
            }
        } else {
            if app
                .attempt_manager_config
                .attempts_table_config
                .attempt_under_cursor
                == i
            {
                Style::new()
            } else {
                if passed_count == total_count {
                    Style::new().fg(Color::LightGreen)
                } else {
                    Style::new().fg(Color::Red)
                }
            }
        };
        let row = Row::new(vec![
            attempt
                .timestamp
                .as_ref()
                .expect("While working with db:")
                .clone(),
            tests_passed,
        ])
        .style(style);

        rows.push(row);
    }

    let header = Row::new(vec!["Дата попытки", "Тесты"]).bottom_margin(1);

    let widths = [Constraint::Percentage(70), Constraint::Percentage(30)];

    let style = match app.attempt_manager_config.status {
        AttemptManagerStatus::SelectingAttempts => Style::default().bg(Color::DarkGray),
        AttemptManagerStatus::SelectingTests => Style::default(),
    };
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL))
        .row_highlight_style(style)
        .highlight_symbol(">> ");

    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("▲"))
        .end_symbol(Some("▼"));

    let scrollbar_area = Rect {
        x: area.x + area.width - 1,
        y: area.y + 1,
        width: 1,
        height: area.height.saturating_sub(2),
    };

    app.attempt_manager_config
        .attempts_table_config
        .attempts_scrollbar_state = app
        .attempt_manager_config
        .attempts_table_config
        .attempts_scrollbar_state
        .content_length(attempts.len())
        .position(
            app.attempt_manager_config
                .attempts_table_config
                .attempts_table_state
                .selected()
                .unwrap_or(0),
        );

    frame.render_stateful_widget(
        table,
        area,
        &mut app
            .attempt_manager_config
            .attempts_table_config
            .attempts_table_state,
    );
    frame.render_stateful_widget(
        scrollbar,
        scrollbar_area,
        &mut app
            .attempt_manager_config
            .attempts_table_config
            .attempts_scrollbar_state,
    );
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for word in text.split_whitespace() {
        let word_width = word.width();

        // Если слово не помещается в текущую строку
        if current_width + word_width + (if current_line.is_empty() { 0 } else { 1 }) > max_width {
            // Сохраняем текущую строку, если она не пустая
            if !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
                current_width = 0;
            }

            // Если слово слишком длинное, разбиваем его на части
            if word_width > max_width {
                let mut remaining = word;
                while !remaining.is_empty() {
                    let mut chunk = String::new();
                    let mut chunk_width = 0;

                    for ch in remaining.chars() {
                        let ch_width = ch.width().unwrap_or(0);
                        if chunk_width + ch_width <= max_width {
                            chunk.push(ch);
                            chunk_width += ch_width;
                        } else {
                            break;
                        }
                    }

                    lines.push(chunk.clone());
                    remaining = &remaining[chunk.len()..];
                }
                continue;
            }
        };

        // Добавляем пробел перед словом
        if !current_line.is_empty() {
            current_line.push(' ');
            current_width += 1;
        }

        current_line.push_str(word);
        current_width += word_width;
    }

    // Добавляем последнюю строку
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
pub fn render_tests_table(frame: &mut Frame, app: &mut App, area: Rect) {
    let tests = app.tests_of_choosed_attempt();
    let passed_count = tests
        .iter()
        .filter(|t| t.result == TestResult::NotExecuted)
        .count();
    let total_count = tests.len();

    let title = if passed_count == total_count {
        format!("Тесты: пройдены все ({}/{})", passed_count, total_count)
    } else {
        format!("Тесты: пройдены не все ({}/{})", passed_count, total_count)
    };
    let items: Vec<ListItem> = tests
        .iter()
        .enumerate()
        .map(|(i, test)| {
            let style =
                if app.attempt_manager_config.status == AttemptManagerStatus::SelectingAttempts {
                    match test.result {
                        TestResult::Passed => Style::default().fg(Color::LightGreen),
                        TestResult::Failed => Style::default().fg(Color::Red),
                        TestResult::NotExecuted => Style::default().fg(Color::DarkGray),
                    }
                } else {
                    if i != app
                        .attempt_manager_config
                        .tests_table_config
                        .test_under_cursor
                    {
                        match test.result {
                            TestResult::Passed => Style::default().fg(Color::LightGreen),
                            TestResult::Failed => Style::default().fg(Color::Red),
                            TestResult::NotExecuted => Style::default().fg(Color::DarkGray),
                        }
                    } else {
                        Style::default()
                    }
                };

            let max_width = area.width.saturating_sub(4) as usize;
            let lines = wrap_text(&test.description, max_width);

            let text_lines: Vec<Line> = lines
                .into_iter()
                .map(|line| Line::from(vec![Span::styled(line, style)]))
                .collect();

            ListItem::new(Text::from(text_lines))
        })
        .collect();

    let style = match app.attempt_manager_config.status {
        AttemptManagerStatus::SelectingTests => Style::default().bg(Color::DarkGray),
        AttemptManagerStatus::SelectingAttempts => Style::default(),
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::White)),
        )
        .highlight_style(style)
        .highlight_symbol(">> ");

    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("▲"))
        .end_symbol(Some("▼"));

    let scrollbar_area = Rect {
        x: area.x + area.width - 1,
        y: area.y + 1,
        width: 1,
        height: area.height.saturating_sub(2),
    };

    app.attempt_manager_config
        .tests_table_config
        .scrollbar_state = app
        .attempt_manager_config
        .tests_table_config
        .scrollbar_state
        .content_length(tests.len())
        .position(
            app.attempt_manager_config
                .tests_table_config
                .list_state
                .selected()
                .unwrap_or(0),
        );
    frame.render_stateful_widget(
        list,
        area,
        &mut app.attempt_manager_config.tests_table_config.list_state,
    );

    frame.render_stateful_widget(
        scrollbar,
        scrollbar_area,
        &mut app
            .attempt_manager_config
            .tests_table_config
            .scrollbar_state,
    );
}
