pub(crate) use crate::ui::Ui;

#[derive(Debug, Default)]
pub struct App {}

impl epi::App for App {
    fn update(&mut self, ctx: &epi::egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let mut ui = Ui::new();
        ui.tick(ctx, frame);
    }

    fn name(&self) -> &str {
        "comik"
    }
}
