use std::path::PathBuf;

use anyhow::Result;
use eframe::egui::SidePanel;
use eframe::egui::Vec2;
use epi::egui;
use epi::egui::Color32;
use epi::egui::CtxRef;
use epi::egui::TextureId;
use image::Pixel;

use crate::providers::file_system::FileSystemCollectionProvider;
use crate::providers::CollectionProvider;

#[derive(Debug, Default)]
pub struct Ui {
    collection: Option<FileSystemCollectionProvider>,
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

            let collection =
                FileSystemCollectionProvider::new("collection name".to_string(), dropped_files)
                    .unwrap();

            self.collection = Some(collection);
        }
        
        egui::SidePanel::left("thumbnail_panel")
            .min_width(200.)
            .max_width(500.)
            .resizable(true)
            .show(ctx, |ui| {
                
                let item = ui.add(&mut widgets::ThumbnailList::new());
                
        });

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

mod widgets {
    use eframe::egui::{Color32, Image, Response, Sense, TextureId, Ui, Vec2, Widget};

    
    
#[derive(Default)]
    pub struct ThumbnailList {
        thumbnail_list: Vec<ThumbnailItem>,
        selected_index: i32,
    }

    impl ThumbnailList {
        pub fn new() -> Self {
            let thumbnail_size = Vec2::new(100., 100.);
            let vec = vec![

                ThumbnailItem::new(0, thumbnail_size),
                ThumbnailItem::new(1, thumbnail_size),
                ThumbnailItem::new(2, thumbnail_size),
                ThumbnailItem::new(3, thumbnail_size),
                ThumbnailItem::new(4, thumbnail_size),



            ];
            Self {
                thumbnail_list: vec,
                selected_index: 0
            }
        }
    }

    impl Widget for &mut ThumbnailList {
        fn ui(self, ui: &mut Ui) -> Response {
            use crate::ui::egui;
            let mut ctx = egui::CtxRef::default();
            let mut scrollarea = egui::ScrollArea::new([true,true])
                .show(ui, |ui| {
                    ui.colored_label(Color32::WHITE, "-- THUMBNAILLIST --");
                
                    let thumbnail_size = Vec2::new(100., 100.);
                     for item in self.thumbnail_list.as_slice() {//for i in thumbs.len
                        let thumbitem = ui.add(&mut ThumbnailItem::new(item.index_number, thumbnail_size));
                        let thumbitem = thumbitem.interact(Sense::click());
                        if thumbitem.clicked() {
                            println!("Item index: {} clicked!", item.index_number);
                        }
                    
                    }
                    
            });
            
            //let panel = ui)
            

            
            

            let total_size = Vec2::new(100.,100.) * Vec2::new(1., self.thumbnail_list.len() as f32);

            let (rect, response) = ui.allocate_exact_size(total_size, Sense::click());
            //self.paint_at(ui, rect);

            //let response = response.interact(Sense::click());

            if response.clicked() {
                println!("Clicked ThumbnailList!");
            }    
            
            response
        }
    }
    
    #[derive(Debug)]
    pub struct ThumbnailItem {
        image: Option<Image>,
        index_number: i32,
        clicked: fn(i32), //fireoff click event
        selected: bool,
        image_size: Vec2
    }

    impl ThumbnailItem {
        pub fn new(current_page: i32, image_size: Vec2) -> Self {
            Self {
                image: None,
                index_number: current_page,
                clicked: |i|{println!("ThumbItem {} clicked", i)},
                selected: false,
                image_size: image_size
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
                selected: false,
                image_size: Vec2::new(100.,100.)
            }
        }
    }

    impl Widget for &mut ThumbnailItem {
        fn ui(self, ui: &mut Ui) -> Response {
            use crate::ui::egui;
            let mut ctx = egui::CtxRef::default();
            
            let image_size = self.image_size;
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

            //let response = response.interact(Sense::click());

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