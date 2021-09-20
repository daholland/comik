#![windows_subsystem = "windows"]

mod ui;
mod comic;

use iced::{Application, Settings};
use ui::app;

fn main() -> iced::Result {
    app::App::run(Settings::default())
}
