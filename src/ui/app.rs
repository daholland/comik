use std::path::PathBuf;

use anyhow::Result;

use iced::{Application, Column, Command, Container, Element, Length, Text};

use crate::comic::{Comic, ComicError};

#[derive(Debug, Default)]
pub struct App {
    focused: bool,
    is_dropping: bool,
    is_opening: bool,
    current_comic: Option<Comic>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FileDropped(PathBuf),
    FileHovered,
    FileHoveredLeft,
    GainedFocus,
    LostFocus,
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
        "comik".to_string()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Message> {
        match message {
            Message::FileHovered => {
                println!("hovering");

                self.is_dropping = true
            },
            Message::FileHoveredLeft => self.is_dropping = false,
            Message::FileDropped(path) => {
                self.is_dropping = false;
                self.is_opening = true;

                println!("dropped {}", path.to_str().unwrap());

                Command::perform(Comic::from_archive_path(path), Message::ComicOpened);
            }
            Message::GainedFocus => self.focused = true,
            Message::LostFocus => self.focused = false,
            Message::ComicOpened(result) => {
                dbg!(&result);
                self.is_opening = false;
                self.current_comic = Some(result.unwrap());
            }
        };

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced_native::subscription::events_with(|event, _status| match event {
            iced_native::Event::Window(window_event) => match window_event {
                iced_native::window::Event::Focused => Some(Message::GainedFocus),
                iced_native::window::Event::Unfocused => Some(Message::LostFocus),
                iced_native::window::Event::FileHovered(_) => Some(Message::FileHovered),
                iced_native::window::Event::FileDropped(path) => Some(Message::FileDropped(path)),
                iced_native::window::Event::FilesHoveredLeft => Some(Message::FileHoveredLeft),
                _ => None,
            },
            _ => None,
        })
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        // let content = Column::new();
        let content = match &self.current_comic {
            Some(comic) => Column::new()
                .width(Length::Shrink)
                .push(Text::new(format!("Opened {}", comic.title))),
            None => Column::new()
                .width(Length::Shrink)
                .push(Text::new("No Comic Loaded")),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
