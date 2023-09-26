use super::Dialog;
use crate::app::message_handling::GuiMes;
use iced::{
    widget::{component, Column, Component},
    Element, Renderer,
};

#[derive(Clone, Debug)]
pub(crate) struct NewHistoryDialog {
    data: NewHistoryData,
}

impl NewHistoryDialog {
    pub(crate) fn new() -> Self {
        NewHistoryDialog {
            data: NewHistoryData {},
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NewHistoryData {}

impl NewHistoryData {}

impl Dialog for NewHistoryDialog {
    fn header(&self) -> String {
        "Create new history item".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }
}

impl Component<GuiMes, Renderer> for NewHistoryDialog {
    type State = ();

    type Event = NewHistoryMes;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        Column::new().padding(5).spacing(5).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewHistoryMes {}
