use super::{CardStyle, Dialog};
use crate::app::message_handling::GuiMes;
use crate::errors::LoreGuiError;
use iced::widget::{component, Component};
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

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }

    fn update(&mut self, _message: super::DialogMessage) {}

    fn submit(&self) -> GuiMes {
        GuiMes::DialogClosed
    }
}

impl Component<GuiMes> for ErrorDialog {
    type State = ();

    type Event = ErrorDialogMes;

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<GuiMes> {
        Some(GuiMes::DialogClosed)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let text = Text::new(self.error.to_string());
        let button = Button::new(Text::new("Ok")).on_press(ErrorDialogMes::Close);
        Column::new().push(text).push(button).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ErrorDialogMes {
    Close,
}
