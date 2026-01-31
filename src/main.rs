mod app;
mod attempt_manager;
mod db;
mod docker;
mod main_menu;
mod popup;
mod pty;
mod task;
mod test;
use crate::app::{App, AppStatus};
use crate::task::TaskStatus;
use ratatui::Frame;
use std::io;

async fn run() -> bool {
    let mut terminal = ratatui::init();
    let _ = color_eyre::install();

    let mut app = App::new();
    let _ = app.run_app(&mut terminal).await;

    match app.status {
        AppStatus::Exiting => false,

        // AppStatus::RunningTask => {
        //     let task = &mut app.config.tasks[app.task_under_cursor];
        //
        //     match create_task_container(&task).await {
        //         Err(err) => eprintln!("{err}"),
        //         _ => {}
        //     };
        //
        //     match app.config.save_config() {
        //         Err(err) => eprintln!("{err}"),
        //         _ => {}
        //     };
        //     true
        // }
        _ => true,
    }
}

#[tokio::main]
async fn main() {
    while run().await {}
    ratatui::restore();
}
