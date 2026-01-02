mod app;
mod docker;
mod ui;
use crate::app::App;
use crate::docker::run_interactive;
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

    if let Some(task) = app.task_to_run {
        run_interactive(&task);
        return true;
    }
    false
}

#[tokio::main]
async fn main() {
    while run().await {}
    // loop {
    //     let mut app = App::new();
    //     let _ = app.run_app(&mut terminal);
    //     if let Some(ref task) = app.task_to_run {
    //         Command::new("clear");
    //         run_interactive("task1");
    //
    //         let _ = color_eyre::install();
    //         let mut terminal = ratatui::init();
    //     }
    // }

    // match docker::build_task_image("task1").await {
    //     Err(err) => eprintln!("{err}"),
    //     Ok(()) => {}
    // }
    //
    // let response = docker::create_task_container("task1").await;
    // if let Err(err) = response {
    //     eprintln!("{err}");
    // }
}
