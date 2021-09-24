#![windows_subsystem = "windows"]

mod comic;
mod app;

use iced::{Application, Settings};

fn main() -> iced::Result {
    app::App::run(Settings::default())
}
