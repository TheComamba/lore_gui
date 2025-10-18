use iced::widget::{text, Text};

pub(super) fn header(content: &str) -> Text<'_> {
    text(content).size(25)
}
