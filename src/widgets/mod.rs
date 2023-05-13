use iced::widget;

use crate::message::Message;

pub trait CustomWidget {
    
}

impl CustomWidget for widget::Button<'_, Message> {}
impl CustomWidget for widget::Checkbox<'_, Message> {}
impl CustomWidget for widget::Column<'_, Message> {}
impl CustomWidget for widget::Container<'_, Message> {}
impl CustomWidget for widget::Image {}
impl CustomWidget for widget::PaneGrid<'_, Message> {}
impl<T> CustomWidget for widget::PickList<'_, T, Message> where [T]: ToOwned<Owned = Vec<T>> {}
impl CustomWidget for widget::ProgressBar<iced::Renderer> {}
impl CustomWidget for widget::Radio<Message> {}
impl CustomWidget for widget::Row<'_, Message> {}
impl CustomWidget for widget::Rule<iced::Renderer> {}
impl CustomWidget for widget::Scrollable<'_, Message> {}
impl<T> CustomWidget for widget::Slider<'_, T, Message, iced::Renderer> {}
impl CustomWidget for widget::Space {}
impl CustomWidget for widget::Text<'_, iced::Renderer> {}
impl CustomWidget for widget::TextInput<'_, Message> {}
impl CustomWidget for widget::Toggler<'_, Message> {}
impl CustomWidget for widget::Tooltip<'_, Message> {}
impl<T> CustomWidget for widget::VerticalSlider<'_, T, Message, iced::Renderer> {}
