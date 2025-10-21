mod app;
mod ui;
use crate::app::App;
use ratatui::{DefaultTerminal, Frame};
use std::io;

fn main() -> io::Result<()> {
    let _ = color_eyre::install();
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);

    ratatui::restore();
    app_result
}
