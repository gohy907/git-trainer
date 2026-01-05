mod app;
mod docker;
mod ui;
use crate::app::{App, Config, Status};
use crate::docker::{build_task_image, create_task_container, run_interactive};
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

    if app.run_task {
        let task = &mut app.config.tasks[app.task_under_cursor];
        // ТОЛЬКО ДЛЯ РАЗРАБОТКИ, УБРАТЬ В ПРОДЕ К ЧЁРТОВОЙ МАТЕРИ
        match build_task_image(&task).await {
            Err(err) => eprintln!("{err}"),
            _ => {}
        };
        match create_task_container(&task).await {
            Err(err) => eprintln!("{err}"),
            Ok(ok) => println!("{ok}"),
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
        return true;
    }
    false
}

#[tokio::main]
async fn main() {
    while run().await {}
}
