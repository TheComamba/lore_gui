use super::Dialog;
use crate::{app::message_handling::GuiMes, errors::LoreGuiError};
use iced::{
    widget::{component, Button, Column, Component, Text, TextInput},
    Element, Renderer,
};
use lorecore::sql::{history::HistoryItem, lore_database::LoreDatabase};

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

impl NewHistoryData {
    pub(crate) fn write_to_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        let item = HistoryItem {
            timestamp: lorecore::timestamp::current_timestamp(),
            year: self.year,
            day: self.day,
            content: self.content,
            properties: self.properties,
        };
        db.write_history_items(vec![item])?;
        Ok(())
    }
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

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            NewHistoryMes::YearUpd(year) => {
                if year.is_empty() {
                    self.data.year = 0;
                } else {
                    let year = year.parse::<i32>();
                    if let Ok(year) = year {
                        self.data.year = year
                    };
                }
                None
            }
            NewHistoryMes::DayUpd(day) => {
                if day.is_empty() {
                    self.data.day = None;
                } else {
                    let day = day.parse::<i32>();
                    if let Ok(day) = day {
                        self.data.day = Some(day)
                    };
                }
                None
            }
            NewHistoryMes::ContentUpd(content) => {
                self.data.content = content;
                None
            }
            NewHistoryMes::Submit => Some(GuiMes::NewHistoryItem(self.data.clone())),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let year_input =
            TextInput::new("", &self.data.year.to_string()).on_input(NewHistoryMes::YearUpd);
        let day_string = match self.data.day {
            Some(day) => day.to_string(),
            None => String::new(),
        };
        let day_input = TextInput::new("", &day_string).on_input(NewHistoryMes::DayUpd);
        let content_input =
            TextInput::new("", &self.data.content).on_input(NewHistoryMes::ContentUpd);
        let submit_button = Button::new("Create").on_press(NewHistoryMes::Submit);
        Column::new()
            .push(Text::new("Year:"))
            .push(year_input)
            .push(Text::new("Day:"))
            .push(day_input)
            .push(Text::new("Content:"))
            .push(content_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewHistoryMes {
    YearUpd(String),
    DayUpd(String),
    ContentUpd(String),
    Submit,
}
