use super::Dialog;
use crate::app::message_handling::GuiMes;
use iced::widget::{component, Button, Column, Component, Text};
use iced::Element;
use iced_aw::CardStyles;

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
    fn card_style(&self) -> CardStyles {
        CardStyles::Warning
    }

    fn header(&self) -> String {
        "Confirmation".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }
}

impl Component<GuiMes> for ConfirmationDialog {
    type State = ();
    type Event = GuiMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            GuiMes::DialogClosed => Some(GuiMes::DialogClosed),
            _ => None,
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let message = Text::new(&self.message);
        let yes_button = Button::new(Text::new("Yes")).on_press(self.on_confirm.clone());
        let no_button = Button::new(Text::new("No")).on_press(GuiMes::DialogClosed);
        Column::new()
            .push(message)
            .push(yes_button)
            .push(no_button)
            .into()
    }
}
