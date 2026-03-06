mod app;
mod attempt_manager;
mod db;
mod docker;
mod main_menu;
mod popup;
mod pty;
use crate::app::{App, AppStatus};
use ratatui::Frame;
use std::io;

async fn run() -> bool {
    let mut terminal = ratatui::init();
    let _ = color_eyre::install();

    let mut app = App::new();
    let _ = app.run_app(&mut terminal).await;

    !matches!(app.status, AppStatus::Exiting)
}

#[tokio::main]
async fn main() {
    while run().await {}
    ratatui::restore();
}
