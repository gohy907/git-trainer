mod app;
mod docker;
mod ui;
use crate::app::{App, AppStatus, TaskStatus};
use crate::docker::{build_task_image, create_task_container, restart_task, run_interactive};
use crate::ui::ui;
use ratatui::prelude::Backend;
use ratatui::{Frame, Terminal};
use std::io;

async fn run() -> bool {
    let _ = color_eyre::install();
    let mut terminal = ratatui::init();

    let mut app = App::new();

    let _ = app.run_app(&mut terminal);

    ratatui::restore();

    match app.status {
        AppStatus::Exiting => false,
        AppStatus::RunningTask => {
            let task = &mut app.config.tasks[app.task_under_cursor];

            match create_task_container(&task).await {
                Err(err) => eprintln!("{err}"),
                _ => {}
            };
            match run_interactive(task) {
                Err(err) => eprintln!("{err}"),
                _ => {}
            };

            match app.config.save_config() {
                Err(err) => eprintln!("{err}"),
                _ => {}
            };
            true
        }
        AppStatus::RestartingTask => {
            let task = &mut app.config.tasks[app.task_under_cursor];
            match restart_task(task).await {
                Err(err) => eprintln!("Error while restarting task: {}", err),
                _ => {}
            };
            true
        }
        AppStatus::Idling => true,
    }
}

#[tokio::main]
async fn main() {
    while run().await {}
}
