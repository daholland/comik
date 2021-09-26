#![windows_subsystem = "windows"]

use anyhow::Result;

mod comic;
mod app;
mod window;
mod ui;

fn main() -> Result<()>{
    app::App::run("comik".to_string())
}
