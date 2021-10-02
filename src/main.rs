#![windows_subsystem = "windows"]

mod app;
mod ui;
mod providers;

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..eframe::NativeOptions::default()
    };

    let app = app::App::new();

    eframe::run_native(Box::new(app), options);
}
