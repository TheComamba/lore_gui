use iced::{
    widget::{component, Button, Column, Component, Text, TextInput},
    Element,
};
use lorecore::{
    errors::LoreCoreError,
    sql::lore_database::LoreDatabase,
    types::{
        day::Day, history::HistoryItem, history_item_content::HistoryItemContent,
        history_item_properties::HistoryItemProperties, year::Year,
    },
};

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::{Dialog, DialogMessage};

#[derive(Clone, Debug)]
pub(crate) struct NewHistoryDialog {
    data: NewHistoryData,
}

impl NewHistoryDialog {
    pub(crate) fn new() -> Self {
        NewHistoryDialog {
            data: NewHistoryData {
                year: 0.into(),
                day: Day::NONE,
                content: "".into(),
                properties: HistoryItemProperties::none(),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NewHistoryData {
    pub(crate) year: Year,
    pub(crate) day: Day,
    pub(crate) content: HistoryItemContent,
    pub(crate) properties: HistoryItemProperties,
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

    fn update(&mut self, message: DialogMessage) {
        match message {
            DialogMessage::YearUpd(year) => {
                if let Ok(year) = year {
                    self.data.year = year;
                }
            }
            DialogMessage::DayUpd(day) => {
                if let Ok(day) = day {
                    self.data.day = day;
                }
            }
            DialogMessage::ContentUpd(content) => {
                self.data.content = content;
            }
            _ => (),
        }
    }

    fn submit(&self) -> GuiMes {
        GuiMes::NewHistoryItem(self.data.clone())
    }
}

impl Component<GuiMes> for NewHistoryDialog {
    type State = ();

    type Event = NewHistoryMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            NewHistoryMes::YearUpd(year) => {
                if let Ok(year) = year {
                    self.data.year = year;
                }
                None
            }
            NewHistoryMes::DayUpd(day) => {
                if let Ok(day) = day {
                    self.data.day = day;
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

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let year_input = TextInput::new("", &self.data.year.to_string())
            .on_input(|i| NewHistoryMes::YearUpd(i.try_into()));
        let day_string = format!("{}", self.data.day);
        let day_input =
            TextInput::new("", &day_string).on_input(|i| NewHistoryMes::DayUpd(i.try_into()));
        let content_input = TextInput::new("", self.data.content.to_str())
            .on_input(|i| NewHistoryMes::ContentUpd(i.into()));
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
    YearUpd(Result<Year, LoreCoreError>),
    DayUpd(Result<Day, LoreCoreError>),
    ContentUpd(HistoryItemContent),
    Submit,
}
