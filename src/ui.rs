use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use epi::egui;
use epi::egui::Color32;
use epi::egui::CtxRef;
use epi::egui::TextureId;
use image::Pixel;

use crate::providers::ComicProvider;
use crate::providers::PageProvider;
use crate::providers::file_system::FileSystemCollectionProvider;
use crate::providers::CollectionProvider;

#[derive(Default)]
pub struct Ui {
    collection: Option<Arc<dyn CollectionProvider>>,
    current_comic: Option<Arc<dyn ComicProvider>>,
    current_page: Option<Arc<dyn PageProvider>>,
    current_comic_index: usize,
    current_page_index: usize,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn tick(&mut self, ctx: &CtxRef, frame: &mut epi::Frame<'_>) {
        if !&ctx.input().raw.dropped_files.is_empty() {
            println!("files dropped");

            let dropped_files = ctx
                .input()
                .raw
                .dropped_files
                .clone()
                .iter()
                .map(|file| file.path.as_ref().unwrap().clone())
                .collect::<Vec<PathBuf>>();

            dbg!(&dropped_files);

            let collection =
                FileSystemCollectionProvider::new("collection name".to_string(), dropped_files)
                    .unwrap();

            self.collection = Some(Arc::new(collection));
            self.current_comic = None;
            self.current_page = None;
            self.current_comic_index = 0;
            self.current_page_index = 0;
        }

        if let Some(collection) = self.collection.clone() {
            if self.current_comic.is_none() {
                let comic = Arc::new(collection.get_comic(self.current_comic_index).unwrap().clone());
                self.current_comic = Some(comic);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            if let Some((texture, size)) = self.render_current_page(frame) {
                dbg!(&texture);
                ui.image(texture, size);
            }
        });
    }

    fn render_current_page(&self, frame: &mut epi::Frame<'_>) -> Option<(TextureId, egui::Vec2)> {
        if let Some(collection) = &self.collection {
            if let Some(comic) = collection.get_comic(self.current_comic_index) {
                let page = comic.get_page(self.current_page_index).unwrap();
                let image = page.get_image().unwrap().to_rgb8();
                let size = (image.width() as usize, image.height() as usize);

                // TODO! apply zoom and panning
                let pixels = image
                    .pixels()
                    .into_iter()
                    .map(|pixel| {
                        let [r, g, b, a] = pixel.to_rgba().0;
                        Color32::from_rgba_unmultiplied(r, g, b, a)
                    })
                    .collect::<Vec<Color32>>();

                let texture = frame
                    .tex_allocator()
                    .alloc_srgba_premultiplied(size, &pixels);

                let size = egui::Vec2::new(size.0 as f32, size.1 as f32);

                return Some((texture, size));
            }
        }

        None
    }
}
