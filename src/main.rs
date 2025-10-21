mod app;
mod ui;
use crate::app::App;
use crate::ui::ui;
use ratatui::prelude::Backend;
use ratatui::{Frame, Terminal};
use std::io;

fn main() {
    let _ = color_eyre::install();
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let _ = app.run_app(&mut terminal);
    ratatui::restore();
}
