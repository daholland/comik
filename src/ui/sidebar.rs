use iced::{Align, Column, Element};

#[derive(Debug, Default, Clone)]
pub struct Sidebar {
    visible: bool
}

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    Toggle
}

impl Sidebar {
    fn new(visible: bool) -> Self {
        Self {
            visible
        }
    }

    fn update(&mut self, message: SidebarMessage) {}

    fn view(&mut self) -> Element<SidebarMessage> {
        Column::new()
            .spacing(20)
            .align_items(Align::Center)
            .into()
    }
}
