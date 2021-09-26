use anyhow::Result;
use epi::egui;

pub struct Ui {}

impl Ui {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn tick(&mut self, ctx: &epi::egui::CtxRef) -> bool {
        let should_quit = false;

        should_quit
    }
}
