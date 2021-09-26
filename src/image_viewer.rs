// based off of https://github.com/iced-rs/iced/blob/master/native/src/widget/image/viewer.rs
use std::hash::Hash;
use iced_native::{Element, Event, Hasher, Layout, Length, Point, Rectangle, Size, Vector, Widget, clipboard::Clipboard, event, layout, mouse};
use iced_graphics::widget::image::viewer;

#[derive(Debug, Clone, Copy)]
pub struct ImageViewerState {
    scale: f32,
    starting_offset: Vector,
    current_offset: Vector,
    cursor_grabbed_at: Option<Point>,
}

impl Default for ImageViewerState {
    fn default() -> Self {
        Self {
            scale: 1.0,
            starting_offset: Vector::default(),
            current_offset: Vector::default(),
            cursor_grabbed_at: None,
        }
    }
}

impl ImageViewerState {
    pub fn new() -> Self {
        ImageViewerState::default()
    }

    fn offset(&self, bounds: Rectangle, image_size: Size) -> Vector {
        let hidden_width = (image_size.width - bounds.width / 2.0).max(0.0).round();

        let hidden_height = (image_size.height - bounds.height / 2.0).max(0.0).round();

        Vector::new(
            self.current_offset.x.min(hidden_width).max(-hidden_width),
            self.current_offset.y.min(hidden_height).max(-hidden_height),
        )
    }

    pub fn is_cursor_grabbed(&self) -> bool {
        self.cursor_grabbed_at.is_some()
    }
}

pub struct ImageViewer<'a> {
    state: &'a mut ImageViewerState,
    padding: u16,
    width: Length,
    height: Length,
    min_scale: f32,
    max_scale: f32,
    scale_step: f32,
    handle: iced::image::Handle,
}

impl<'a> ImageViewer<'a> {
    pub fn new(state: &'a mut ImageViewerState, handle: iced::image::Handle) -> Self {
        ImageViewer {
            state,
            padding: 0,
            width: Length::Shrink,
            height: Length::Shrink,
            min_scale: 0.25,
            max_scale: 10.0,
            scale_step: 0.10,
            handle,
        }
    }

    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn min_scale(mut self, min_scale: f32) -> Self {
        self.min_scale = min_scale;
        self
    }

    pub fn scale_step(mut self, scale_step: f32) -> Self {
        self.scale_step = scale_step;
        self
    }

    fn image_size<Renderer>(&self, renderer: &Renderer, bounds: Size) -> Size
    where
        Renderer: self::Renderer + iced_native::image::Renderer,
    {
        let (width, height) = renderer.dimensions(&self.handle);

        let (width, height) = {
            let dimensions = (width as f32, height as f32);

            let width_ratio = bounds.width / dimensions.0;
            let height_ratio = bounds.height / dimensions.1;

            let ratio = width_ratio.min(height_ratio);

            let scale = self.state.scale;

            if ratio < 1.0 {
                (dimensions.0 * ratio * scale, dimensions.1 * ratio * scale)
            } else {
                (dimensions.0 * scale, dimensions.1 * scale)
            }
        };

        Size::new(width, height)
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for ImageViewer<'a>
where
    Renderer: self::Renderer + iced_native::image::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let (width, height) = renderer.dimensions(&self.handle);

        let mut size = limits
            .width(self.width)
            .height(self.height)
            .resolve(Size::new(width as f32, height as f32));

        let expansion_size = if height > width {
            self.width
        } else {
            self.height
        };

        // Only calculate viewport sizes if the images are constrained to a limited space.
        // If they are Fill|Portion let them expand within their alotted space.
        match expansion_size {
            Length::Shrink | Length::Units(_) => {
                let aspect_ratio = width as f32 / height as f32;
                let viewport_aspect_ratio = size.width / size.height;
                if viewport_aspect_ratio > aspect_ratio {
                    size.width = width as f32 * size.height / height as f32;
                } else {
                    size.height = height as f32 * size.width / width as f32;
                }
            }
            Length::Fill | Length::FillPortion(_) => {}
        }

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _messages: &mut Vec<Message>,
    ) -> event::Status {
        let bounds = layout.bounds();
        let is_mouse_over = bounds.contains(cursor_position);

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) if is_mouse_over => {
                match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                        let previous_scale = self.state.scale;

                        if y < 0.0 && previous_scale > self.min_scale
                            || y > 0.0 && previous_scale < self.max_scale
                        {
                            self.state.scale = (if y > 0.0 {
                                self.state.scale * (1.0 + self.scale_step)
                            } else {
                                self.state.scale / (1.0 + self.scale_step)
                            })
                            .max(self.min_scale)
                            .min(self.max_scale);

                            let image_size = self.image_size(renderer, bounds.size());

                            let factor = self.state.scale / previous_scale - 1.0;

                            let cursor_to_center = cursor_position - bounds.center();

                            let adjustment =
                                cursor_to_center * factor + self.state.current_offset * factor;

                            self.state.current_offset = Vector::new(
                                if image_size.width > bounds.width {
                                    self.state.current_offset.x + adjustment.x
                                } else {
                                    0.0
                                },
                                if image_size.height > bounds.height {
                                    self.state.current_offset.y + adjustment.y
                                } else {
                                    0.0
                                },
                            );
                        }
                    }
                }

                event::Status::Captured
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if is_mouse_over => {
                self.state.cursor_grabbed_at = Some(cursor_position);
                self.state.starting_offset = self.state.current_offset;

                event::Status::Captured
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if self.state.cursor_grabbed_at.is_some() =>
            {
                self.state.cursor_grabbed_at = None;

                event::Status::Captured
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Some(origin) = self.state.cursor_grabbed_at {
                    let image_size = self.image_size(renderer, bounds.size());

                    let hidden_width = (image_size.width - bounds.width / 2.0).max(0.0).round();

                    let hidden_height = (image_size.height - bounds.height / 2.0).max(0.0).round();

                    let delta = position - origin;

                    let x = if bounds.width < image_size.width {
                        (self.state.starting_offset.x - delta.x)
                            .min(hidden_width)
                            .max(-hidden_width)
                    } else {
                        0.0
                    };

                    let y = if bounds.height < image_size.height {
                        (self.state.starting_offset.y - delta.y)
                            .min(hidden_height)
                            .max(-hidden_height)
                    } else {
                        0.0
                    };

                    self.state.current_offset = Vector::new(x, y);

                    event::Status::Captured
                } else {
                    event::Status::Ignored
                }
            }
            _ => event::Status::Ignored,
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        let bounds = layout.bounds();

        let image_size = self.image_size(renderer, bounds.size());

        let translation = {
            let image_top_left = Vector::new(
                bounds.width / 2.0 - image_size.width / 2.0,
                bounds.height / 2.0 - image_size.height / 2.0,
            );

            image_top_left - self.state.offset(bounds, image_size)
        };

        let is_mouse_over = bounds.contains(cursor_position);

        self::Renderer::draw(
            renderer,
            &self.state,
            bounds,
            image_size,
            translation,
            self.handle.clone(),
            is_mouse_over,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
        self.padding.hash(state);

        self.handle.hash(state);
    }
}

/// The renderer of an [`Viewer`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Viewer`] in your user interface.
///
/// [renderer]: crate::renderer
pub trait Renderer: iced_native::Renderer + Sized {
    /// Draws the [`Viewer`].
    ///
    /// It receives:
    /// - the [`State`] of the [`Viewer`]
    /// - the bounds of the [`Viewer`] widget
    /// - the [`Size`] of the scaled [`Viewer`] image
    /// - the translation of the clipped image
    /// - the [`Handle`] to the underlying image
    /// - whether the mouse is over the [`Viewer`] or not
    ///
    /// [`Handle`]: image::Handle
    fn draw(
        &mut self,
        state: &ImageViewerState,
        bounds: Rectangle,
        image_size: Size,
        translation: Vector,
        handle: iced::image::Handle,
        is_mouse_over: bool,
    ) -> Self::Output;
}
