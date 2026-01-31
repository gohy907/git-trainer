use ratatui::style::Color;

#[derive(Clone)]
pub struct Test {
    pub id: usize,
    pub description: String,
    pub status: TestStatus,
}

#[derive(Clone, PartialEq)]
pub enum TestStatus {
    Passed,
    Failed,
    NotRun,
}

impl TestStatus {
    pub fn color(&self) -> Color {
        match self {
            TestStatus::Passed => Color::Green,
            TestStatus::Failed => Color::Red,
            TestStatus::NotRun => Color::Gray,
        }
    }
}
