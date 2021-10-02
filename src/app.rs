use std::{default, sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
}};

use crate::providers::CollectionProvider;
pub(crate) use crate::ui::Ui;

#[derive(Default)]
pub struct AppState {
    comic_collection: Option<Box<dyn CollectionProvider>>,
    collection_index: usize,
    page_index: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

pub struct App {
    state: Arc<Mutex<AppState>>,
    col_tx: Sender<Box<dyn CollectionProvider>>,
    col_rx: Receiver<Box<dyn CollectionProvider>>,
    ui: Ui,
}

impl App {
    pub fn new() -> Self {
        let (col_tx, col_rx): (
            Sender<Box<dyn CollectionProvider>>,
            Receiver<Box<dyn CollectionProvider>>,
        ) = mpsc::channel();

        let appstate = AppState {
            comic_collection: None,
            collection_index: 0,
            page_index: 0,
        };

        let ui = Ui::new();

        Self {
            state: Arc::new(Mutex::new(appstate)),
            ui,
            col_tx,
            col_rx,
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
