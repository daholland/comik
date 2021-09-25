#![windows_subsystem = "windows"]

mod comic;
mod app;
mod image_viewer;

use iced::{Application, Settings};

fn main() -> iced::Result {
    app::App::run(Settings::default())
}
