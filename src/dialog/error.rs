use super::{CardStyle, Dialog};
use crate::app::message_handling::GuiMessage;
use crate::errors::LoreGuiError;
use iced::{
    widget::{Button, Column, Text},
    Element,
};

#[derive(Debug, Clone)]
pub(crate) struct ErrorDialog {
    error: LoreGuiError,
}

impl ErrorDialog {
    pub(crate) fn new(error: LoreGuiError) -> Self {
        ErrorDialog { error }
    }
}

impl Dialog for ErrorDialog {
    fn card_style(&self) -> CardStyle {
        CardStyle::Error
    }

    fn header(&self) -> String {
        "Error".to_string()
    }

    fn body(&self) -> Element<'_, GuiMessage> {
        let text = Text::new(self.error.to_string());
        let button = Button::new(Text::new("Ok")).on_press(GuiMessage::DialogClosed);
        Column::new().push(text).push(button).into()
    }

    fn update(&mut self, _message: super::DialogUpdate) {}

    fn submit(&self) -> GuiMessage {
        GuiMessage::DialogClosed
    }
}
