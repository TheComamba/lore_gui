use super::{CardStyle, Dialog};
use crate::app::message_handling::GuiMes;
use iced::widget::{Button, Column, Row, Text};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub(crate) struct ConfirmationDialog {
    message: String,
    on_confirm: GuiMes,
}

impl ConfirmationDialog {
    pub(crate) fn new(message: String, on_confirm: GuiMes) -> Self {
        Self {
            message,
            on_confirm,
        }
    }
}

impl Dialog for ConfirmationDialog {
    fn card_style(&self) -> CardStyle {
        CardStyle::Warning
    }

    fn header(&self) -> String {
        "Confirmation".to_string()
    }

    fn body<'a>(&'a self) -> Element<'a, GuiMes> {
        let message = Text::new(&self.message);
        let yes_button = Button::new(Text::new("Yes")).on_press(self.on_confirm.clone());
        let no_button = Button::new(Text::new("No")).on_press(GuiMes::DialogClosed);

        let buttons = Row::new().push(yes_button).push(no_button).spacing(50);

        Column::new()
            .push(message)
            .push(buttons)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn update(&mut self, _message: super::DialogMessage) {}

    fn submit(&self) -> GuiMes {
        self.on_confirm.clone()
    }
}
