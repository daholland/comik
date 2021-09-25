use iced::Svg;
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
    layout, mouse, Background, Color, Element, Hasher, Layout, Length,
    Point, Rectangle, Size, Widget,
};

pub struct Thumbnail {

}

pub struct ThumbnailPicker {
    pub image: Svg,
    pub picked_thumbnail_index: u32,
    pub size: (u32, u32)//w/H
}

impl ThumbnailPicker {
    pub fn new() -> Self {
        Self { 
            image: Svg::from_path(
                format!("{}/image.svg", env!("CARGO_MANIFEST_DIR"))),
                picked_thumbnail_index: 0,
                size: (20,20)
        }
    }
}

impl<Message, B> Widget<Message, Renderer<B>> for ThumbnailPicker
    where
        B: Backend,
    {
        fn width(&self) -> Length {
            Length::Shrink
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(
            &self,
            _renderer: &Renderer<B>,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::new(10.0, 10.0))
        }

        fn hash_layout(&self, state: &mut Hasher) {
            use std::hash::Hash;

            self.size.hash(state);
        }

        fn draw(
            &self,
            _renderer: &mut Renderer<B>,
            _defaults: &Defaults,
            layout: Layout<'_>,
            _cursor_position: Point,
            _viewport: &Rectangle,
        ) -> (Primitive, mouse::Interaction) {
            (
                Primitive::Quad {
                    bounds: layout.bounds(),
                    background: Background::Color(Color::WHITE),
                    border_radius: 1.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                mouse::Interaction::default(),
            )
        }
    }

impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for ThumbnailPicker
    where
        B: Backend,
    {
        fn into(self) -> Element<'a, Message, Renderer<B>> {
            Element::new(self)
        }
    }

