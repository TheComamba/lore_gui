use iced::{
    widget::{Container, Text},
    Element, Length,
};

use crate::app::message_handling::GuiMessage;

pub(crate) fn content_view<'a>(text: String) -> Element<'a, GuiMessage> {
    let parsed = text.trim();
    Container::new(Text::new(parsed.to_string()))
        .width(Length::Fill)
        .into()
}
