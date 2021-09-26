use std::path::PathBuf;

use anyhow::Result;
use epi::egui;
use epi::egui::Color32;
use epi::egui::CtxRef;
use epi::egui::TextureId;
use image::Pixel;

use crate::comic::{Comic, ComicCollection};

#[derive(Default, Debug)]
pub struct Ui {
    collection: Option<ComicCollection>,
    current_comic: Option<Comic>,
    current_page_index: i32,
}

impl Ui {
    pub fn new() -> Result<Self> {
        Ok(Self {
            ..Default::default()
        })
    }

    pub fn tick(&mut self, ctx: &CtxRef, frame: &mut epi::Frame<'_>) {
        dbg!(&ctx.input().raw.dropped_files);

        if !ctx.input().raw.dropped_files.is_empty() {
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

            let collection = ComicCollection::new(dropped_files).unwrap();

            if collection.paths.len() > 0 {
                let comic_path = collection.paths.get(0).unwrap().clone();
                let comic = Comic::from_archive_path(comic_path).unwrap();

                self.current_comic = Some(comic);
            }

            self.collection = Some(collection);
        }

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            if let Some((texture, size)) = self.render_current_page(frame) {
                ui.image(texture, size);
            }
        });
    }

    fn render_current_page(&self, frame: &mut epi::Frame<'_>) -> Option<(TextureId, egui::Vec2)> {
        if let Some(comic) = &self.current_comic {
            let page = comic.pages.get(self.current_page_index as usize).unwrap();
            let image = page.as_image().unwrap().to_rgb8();
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

        None
    }
}
