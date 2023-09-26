use super::Dialog;
use crate::app::message_handling::GuiMes;
use iced::{
    widget::{component, Column, Component, Text, TextInput},
    Element, Renderer,
};

#[derive(Clone, Debug)]
pub(crate) struct NewHistoryDialog {
    data: NewHistoryData,
}

impl NewHistoryDialog {
    pub(crate) fn new() -> Self {
        NewHistoryDialog {
            data: NewHistoryData {
                year: 0,
                day: None,
                content: String::new(),
                properties: None,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NewHistoryData {
    pub(crate) year: i32,
    pub(crate) day: Option<i32>,
    pub(crate) content: String,
    pub(crate) properties: Option<String>,
}

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
        let year_input = TextInput::new("", &self.data.year.to_string());
        let day_string = match self.data.day {
            Some(day) => day.to_string(),
            None => String::new(),
        };
        let day_input = TextInput::new("", &day_string);
        let content_input = TextInput::new("", &self.data.content);
        Column::new()
            .push(Text::new("Year:"))
            .push(year_input)
            .push(Text::new("Day:"))
            .push(day_input)
            .push(Text::new("Content:"))
            .push(content_input)
            .padding(5)
            .spacing(5)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewHistoryMes {}
