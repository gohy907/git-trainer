mod app;
mod docker;
mod ui;
use crate::app::{App, Status};
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

    match app.task_to_run {
        Some(task_index) => {
            let task = &mut app.config.tasks[task_index];
            // ТОЛЬКО ДЛЯ РАЗРАБОТКИ, УБРАТЬ В ПРОДЕ К ЧЁРТОВОЙ МАТЕРИ
            match build_task_image(&task).await {
                Err(err) => eprintln!("{err}"),
                _ => {}
            };
            match create_task_container(&task).await {
                Err(err) => eprintln!("{err}"),
                _ => {}
            };

            if task.status == Status::NotInProgress {
                task.status = Status::InProgress;
            }

            match run_interactive(&task) {
                Err(err) => eprintln!("{err}"),
                _ => {}
            };

            match app.config.save_config() {
                Err(err) => eprintln!("{err}"),
                _ => {}
            };
            true
        }
        None => false,
    }
}

#[tokio::main]
async fn main() {
    while run().await {}
}
