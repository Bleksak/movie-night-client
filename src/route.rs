use iced::widget::{column, button, text};

use crate::message::{Message, MessageType};

pub trait Route {
    fn view(&self) -> iced::Element<'_, Message>;
    fn update(&mut self, message: Message);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum RouteKey {
    Home
}

impl Default for RouteKey {
    fn default() -> Self {
        Self::Home
    }
}

pub struct Home(i32);

impl Default for Home {
    fn default() -> Self {
        Self(0)
    }
}

impl Route for Home {
    fn view(&self) -> iced::Element<Message> {
        column![
            button("+").on_press(Message::new(MessageType::Increment)),
            text(self.0).size(50),
            button("-").on_press(Message::new(MessageType::Decrement)),
        ]
        .into()
    }
    
    fn update(&mut self, message: Message) {
        match message.message_type() {
            MessageType::Increment => self.0 += 1,
            MessageType::Decrement => self.0 -= 1,
            _ => {}
        }
    }
}
