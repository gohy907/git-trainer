mod app;
mod docker;
mod tty;
mod ui;
use crate::app::{App, AppStatus, TaskStatus};
use crate::docker::{build_task_image, create_task_container, restart_task, run_interactive};
use crate::ui::ui;
use ratatui::prelude::Backend;
use ratatui::{DefaultTerminal, Frame, Terminal};
use std::io;

async fn run() -> bool {
    let mut terminal = ratatui::init();
    let _ = color_eyre::install();

    let mut app = App::new();

    let _ = app.run_app(&mut terminal).await;

    match app.status {
        AppStatus::Exiting => false,

        AppStatus::RunningTask => {
            let task = &mut app.config.tasks[app.task_under_cursor];

            match create_task_container(&task).await {
                Err(err) => eprintln!("{err}"),
                _ => {}
            };

            match app.config.save_config() {
                Err(err) => eprintln!("{err}"),
                _ => {}
            };
            true
        }
        _ => true,
    }
}

#[tokio::main]
async fn main() {
    while run().await {}
    ratatui::restore();
}
