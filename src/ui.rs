use std::path::PathBuf;

use anyhow::Result;
use eframe::egui::SidePanel;
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
        if ctx.input().key_pressed(egui::Key::Q) {
            
        }
        //dbg!(&ctx.input().raw.dropped_files);

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


        
        egui::SidePanel::left("thumbnail_panel")
            .min_width(200.)
            .max_width(500.)
            .show(ctx, |ui| {
                
                let item = ui.add(&mut widgets::ThumbnailItem::default());
                
        });

        egui::CentralPanel::default()
            .show(ctx, |ui: &mut egui::Ui| {
                if let Some((texture, size)) = self.render_current_page(frame) {
                    ui.image(texture, size);
                }
                ui.colored_label(Color32::WHITE, "Center Panel");
                ui.image(TextureId::Egui, egui::Vec2::new(200., 200.));
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

mod widgets {
    use eframe::egui::{Image, Response, Sense, TextureId, Ui, Vec2, Widget};
    

   
    #[derive(Debug)]
    pub struct ThumbnailItem {
        image: Option<Image>,
        index_number: i32,
        clicked: fn(i32), //fireoff click event
        selected: bool
    }

    impl ThumbnailItem {
        pub fn new(current_page: i32) -> Self {
            Self {
                image: None,
                index_number: 0,
                clicked: |i|{println!("ThumbItem {} clicked", i)},
                selected: false
            }
        }

        pub fn clicked(&mut self,index: i32) {
            println!("ThumbItem index: {}| Clicked", index);
        }
    }

    impl Default for ThumbnailItem {
        fn default() -> Self {
            Self {
                image: None,
                index_number: 0,
                clicked: |i|{ println!("ThumbItem {} clicked", i)},
                selected: false
            }
        }
    }

    impl Widget for &mut ThumbnailItem {
        fn ui(self, ui: &mut Ui) -> Response {
            use crate::ui::egui;
            let mut ctx = egui::CtxRef::default();
            
            let image_size = Vec2::new(25., 25.);
            let image = ui.add(Image::new(TextureId::Egui, image_size));
            let mut image = image.interact(Sense::click());
            if image.clicked() {
                image.rect = image.rect.translate(Vec2::new(10., 10.));
                println!("Image clicked!");
            }
            
            ui.label(self.index_number.to_string());

            let total_size = image_size + Vec2::new(15.,15.);

            let (rect, response) = ui.allocate_exact_size(total_size, Sense::click());
            //self.paint_at(ui, rect);

            let response = response.interact(Sense::click());

            if response.clicked() {
                self.clicked(self.index_number);
            }    
            
            response
            }
        }
    
}

mod math_helpers {
    use std::f64::consts::PI as MathPI;
    struct Tween {
        //function
        funconce: fn(f64,f64) -> f64,
        start: f32, 
        end: f32
    }

    fn easeInSine(x: f64) -> f64 {
        1. - (x * MathPI).cos()
    }

    fn easeOutSine(x: f64) -> f64 {
        unimplemented!()//1. - (x * MathPI).cos()
    }

    impl Tween {
        fn new() -> Self {
            Self {
                funconce: |input, step| {input},
                start: 0f32,
                end: 0f32
            }
        }

    
    }

    
}