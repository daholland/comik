use std::collections::VecDeque;
use std::ops::IndexMut;
use std::path::PathBuf;
use std::sync::Arc;

use epi::egui;
use epi::egui::Color32;
use epi::egui::CtxRef;
use epi::egui::TextureId;
use egui::ScrollArea;
use image::Pixel;

use crate::providers::file_system::FileSystemCollectionProvider;
use crate::providers::CollectionProvider;

pub enum Event {
    ChangePageEvent(usize),
    QuitAppEvent(usize)
}

#[derive(Default)]
pub struct Ui {
    collection: Option<Arc<dyn CollectionProvider>>,
    current_comic_index: usize,
    current_page_index: usize,
    current_image: Option<image::DynamicImage>,
    current_texture: Option<TextureId>,
    current_image_size: egui::Vec2,
    current_image_thumbnails: Vec<image::DynamicImage>,
    event_queue: VecDeque<Event>
}

impl Ui {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn tick(&mut self, ctx: &CtxRef, frame: &mut epi::Frame<'_>) {
        let cargopath: PathBuf = env!("CARGO_MANIFEST_DIR").into();   
        let _hardcoded = vec![cargopath.join("little-nemo-all-421.zip")];

        let redraw = false;
        
        if !&ctx.input().raw.dropped_files.is_empty() || self.collection.is_none()  {
            let dropped_files = ctx
            .input()
            .raw
            .dropped_files
            .clone()
            .iter()
            .map(|file| file.path.as_ref().unwrap().clone())
            .collect::<Vec<PathBuf>>();
            
            dbg!(&_hardcoded);
            let collection =
                FileSystemCollectionProvider::new("collection name".to_string(), _hardcoded)
                    .unwrap();

            self.collection = Some(Arc::new(collection));
            self.current_comic_index = 0;
            self.current_page_index = 0;
        }


        if ctx.input().key_pressed(egui::Key::ArrowRight) {
            dbg!("right pressed");
            if let Some(collection) = self.collection.clone() {
                
                    if let Some(comic) = *collection.get_comic(self.current_comic_index) {
                        dbg!(comic.get_length());
                        if  self.current_page_index < comic.get_length() - 1 {
                            dbg!("push event");
                            self.event_queue.push_back(Event::ChangePageEvent(self.current_page_index + 1))
                        }
                    }
                
            }
            dbg!("end of keypress");
            // dbg!(self.event_queue);
            dbg!(self.current_page_index);
            
        }

        if ctx.input().key_pressed(egui::Key::ArrowLeft) {
            dbg!("left pressed");
            if let Some(collection) = self.collection.clone() {
                
                    if let Some(comic) = *collection.get_comic(self.current_comic_index) {
                        dbg!(comic.get_length());
                        if  self.current_page_index > 0 {
                            dbg!("push event LEFT -> CPE");
                            self.event_queue.push_back(Event::ChangePageEvent(self.current_page_index - 1))
                        }
                    }
                
            }
            dbg!("end of keypress");
            // dbg!(self.event_queue);
            dbg!(self.current_page_index);
            
        }

        //todo: probably want to limit this per frame but theres not many events..?
        while let Some(ev) = self.event_queue.pop_front() {
            match ev {
                Event::ChangePageEvent(new_index) => {
                    
                    self.current_page_index = new_index;
                    self.current_image = None;
                    self.current_texture = None;
                    if let Some(texture) = self.current_texture {
                        
                    }
                        
                dbg!("changepageevent handled");
                
                
                }
                _ => unimplemented!()
            }
        }


        if let Some(collection) = self.collection.clone() {
            if self.current_image.is_none() {
                if let Some(comic) = *collection.get_comic(self.current_comic_index) {
                    if let Some(image_provider) = comic.get_page(self.current_page_index) {
                        let comic_image = image_provider.get_image();
                        self.current_image = Some(comic_image);
                    }
                }
            }
        }
        
        egui::SidePanel::left("thumbnail_panel")
            .min_width(200.)
            .max_width(500.)
            .resizable(true)
            .show(ctx, |ui| {
                
                let item = ui.add(&mut widgets::ThumbnailList::new());
                
        });

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            if let Some(texture_id) = self.current_texture {
                ui.image(texture_id, self.current_image_size);
            } else {
                if let Some((texture, size)) = self.render_current_page(frame) {
                    self.current_texture = Some(texture);
                    self.current_image_size = size;
                    // if (redraw) {
                    //     ui.image(self.current_texture.unwrap(), self.current_image_size);
                    // }
                }
            }

        });

    }

    fn render_current_page(&self, frame: &mut epi::Frame<'_>) -> Option<(TextureId, egui::Vec2)> {
        if let Some(comic_image) = &self.current_image {
            let image = comic_image.to_rgb8();
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
        } else {
            None
        }
    }
}

mod widgets {
    use egui::{Color32, Image, Response, Sense, TextureId, Ui, Vec2, Widget};

    
    
#[derive(Default)]
    pub struct ThumbnailList {
        thumbnail_list: Vec<ThumbnailItem>,
        selected_index: i32,
    }

    impl ThumbnailList {
        pub fn new() -> Self {
            let thumbnail_size = Vec2::new(100., 100.);
            let vec = vec![

                ThumbnailItem::new(0, thumbnail_size, false),
                ThumbnailItem::new(1, thumbnail_size, false),
                ThumbnailItem::new(2, thumbnail_size, true),
                ThumbnailItem::new(3, thumbnail_size, false),
                ThumbnailItem::new(4, thumbnail_size, true),



            ];
            Self {
                thumbnail_list: vec,
                selected_index: 0
            }
        }
    }

    impl Widget for &mut ThumbnailList {
        fn ui(self, ui: &mut Ui) -> Response {
            
            let mut ctx = egui::CtxRef::default();
            let screen = ctx.available_rect();
            let mut scrollarea = egui::ScrollArea::from_max_height(screen.height()).show(ui, |ui| {
                    ui.colored_label(Color32::WHITE, "-- THUMBNAILLIST --");
                
                    let thumbnail_size = Vec2::new(100., 100.);
                     for item in self.thumbnail_list.as_slice() {//for i in thumbs.len
                        let thumbitem = ui.add(&mut ThumbnailItem::new(item.index_number, thumbnail_size, item.selected));
                        let thumbitem = thumbitem.interact(Sense::click());
                        if thumbitem.clicked() {
                            println!("Item index: {} clicked!", item.index_number);
                        }
                    
                    }
                    
            });
            
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
        pub fn new(current_page: i32, image_size: Vec2, selected: bool) -> Self {
            Self {
                image: None,
                index_number: current_page,
                clicked: |i|{println!("ThumbItem {} clicked", i)},
                selected: selected,
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
            let total_size = image_size + Vec2::new(15.,15.);
            //let response = ui.allocate_ui(total_size, |ui| {
                let image = ui.add(Image::new(TextureId::Egui, image_size));
                let _label = ui.label(self.index_number.to_string());
                if self.selected {
                    ui.painter().rect_stroke(image.rect, 0.0, (1.0, egui::Color32::GREEN));
                 }
                let image = image.interact(Sense::click());
                let _label = _label.interact(Sense::click());
                
                if image.clicked() || _label.clicked() {
                    //image.rect = image.rect.translate(Vec2::new(10., 10.));
                    println!("Image {} Clicked! Selected: {}", self.index_number, self.selected);

                }
                
                
                
                let response = image.interact(Sense::click());
                
                //});
            //self.paint_at(ui, rect);


            if response.clicked() {
                self.clicked(self.index_number);
                

            }    
            
            response
            }
        }


}
