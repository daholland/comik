use std::path::PathBuf;

use anyhow::Result;

use iced::{Application, Column, Command, Container, Element, Length, Row, Text};

use crate::{comic::{Comic, ComicError, Page}, image_viewer};

#[derive(Debug, Default)]
pub struct App {
    focused: bool,
    is_dropping: bool,
    is_opening: bool,
    current_comic: Option<Comic>,
    current_page_index: i32,
    current_page_view: Option<PageView>,
}

#[derive(Debug, Clone)]
pub enum WindowMessage {
    FileDropped(PathBuf),
    FileHovered,
    FileHoveredLeft,
    GainedFocus,
    LostFocus,
}

#[derive(Debug, Clone)]
pub enum ComicMessage {
    NextPage,
    PreviousPage,
}

#[derive(Debug, Clone)]
pub enum Message {
    WindowMessage(WindowMessage),
    ComicMessage(ComicMessage),
    ComicOpened(Result<Comic, ComicError>),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (App::default(), Command::none())
    }

    fn title(&self) -> String {
        match &self.current_comic {
            Some(comic) => format!("comik - {}", comic.title),
            None => "comik".to_string(),
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Message> {
        match message {
            Message::WindowMessage(window_message) => match window_message {
                WindowMessage::FileHovered => {
                    self.is_dropping = true;
                }
                WindowMessage::FileHoveredLeft => {
                    self.is_dropping = false;
                }
                WindowMessage::FileDropped(path) => {
                    self.is_dropping = false;
                    self.is_opening = true;
                    self.current_comic = None;

                    return Command::perform(Comic::from_archive_path(path), Message::ComicOpened);
                }
                WindowMessage::GainedFocus => {
                    self.focused = true;
                }
                WindowMessage::LostFocus => {
                    self.focused = false;
                }
            },
            Message::ComicMessage(comic_message) => match comic_message {
                ComicMessage::NextPage => {
                    if let Some(current_comic) = &self.current_comic {
                        let page_count = current_comic.pages.len();
                        if self.current_page_index + 1 != (page_count as i32) {
                            self.current_page_index += 1;

                            let new_page = current_comic
                                .pages
                                .get(self.current_page_index as usize)
                                .unwrap()
                                .clone();

                            self.current_page_view = Some(PageView::open_page(new_page).unwrap());
                        }
                    }
                }
                ComicMessage::PreviousPage => {
                    if let Some(current_comic) = &self.current_comic {
                        if self.current_page_index >= 1 {
                            self.current_page_index -= 1;

                            let new_page = current_comic
                                .pages
                                .get(self.current_page_index as usize)
                                .unwrap()
                                .clone();

                            self.current_page_view = Some(PageView::open_page(new_page).unwrap());
                        }
                    }
                }
            },
            Message::ComicOpened(result) => {
                self.is_opening = false;

                let comic = result.unwrap();

                let new_page = comic
                    .pages
                    .get(self.current_page_index as usize)
                    .unwrap()
                    .clone();

                self.current_page_view = Some(PageView::open_page(new_page).unwrap());
                self.current_comic = Some(comic);
            }
        };

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced_native::subscription::events_with(|event, _status| match event {
            iced_native::Event::Window(window_event) => match window_event {
                iced_native::window::Event::Focused => {
                    Some(Message::WindowMessage(WindowMessage::GainedFocus))
                }
                iced_native::window::Event::Unfocused => {
                    Some(Message::WindowMessage(WindowMessage::LostFocus))
                }
                iced_native::window::Event::FileHovered(_) => {
                    Some(Message::WindowMessage(WindowMessage::FileHovered))
                }
                iced_native::window::Event::FileDropped(path) => {
                    Some(Message::WindowMessage(WindowMessage::FileDropped(path)))
                }
                iced_native::window::Event::FilesHoveredLeft => {
                    Some(Message::WindowMessage(WindowMessage::FileHoveredLeft))
                }
                _ => None,
            },
            _ => None,
        })
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = match &mut self.current_page_view {
            Some(page_view) => Row::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .push(page_view.view()),
            None => match self.is_opening {
                true => Row::new()
                    .width(Length::Shrink)
                    .push(Text::new("Loading Comic File")),
                false => Row::new()
                    .width(Length::Shrink)
                    .push(Text::new("No Comic Loaded")),
            },
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
struct PageView {
    image_viewer: image_viewer::ImageViewerState,
    img_data: iced::image::Handle,
}

impl PageView {
    fn view(&mut self) -> Element<Message> {
        Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .push(
                image_viewer::ImageViewer::new(&mut self.image_viewer, self.img_data.clone())
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .into()
    }

    fn open_page(page: Page) -> Result<PageView> {
        let img_data = iced::image::Handle::from_memory(page.as_bytes().unwrap());
        let image_viewer = image_viewer::ImageViewerState::new();

        Ok(Self {
            image_viewer,
            img_data,
        })
    }
}
