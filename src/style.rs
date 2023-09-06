use iced::widget::{text, Text};

pub(super) fn header(content: &str) -> Text {
    text(content).size(25)
}
