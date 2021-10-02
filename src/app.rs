pub(crate) use crate::ui::Ui;

#[derive(Default)]
pub struct AppState {}


pub struct App {
    ui: Ui,
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: Ui::new()
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "comik"
    }

    fn update(&mut self, ctx: &epi::egui::CtxRef, frame: &mut epi::Frame<'_>) {
        self.ui.tick(ctx, frame);
    }

    fn clear_color(&self) -> eframe::egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        eframe::egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }
}
