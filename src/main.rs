#![windows_subsystem = "windows"]

mod comic;
mod app;
mod ui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app::App::default()), options);
}
