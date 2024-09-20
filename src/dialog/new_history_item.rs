use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{
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

    fn body(&self) -> Element<'_, GuiMes> {
        let year_input = TextInput::new("", &self.data.year.to_string())
            .on_input(|i| GuiMes::DialogUpdate(DialogMessage::YearUpd(i.try_into())));
        let day_string = format!("{}", self.data.day);
        let day_input = TextInput::new("", &day_string)
            .on_input(|i| GuiMes::DialogUpdate(DialogMessage::DayUpd(i.try_into())));
        let content_input = TextInput::new("", self.data.content.to_str())
            .on_input(|i| GuiMes::DialogUpdate(DialogMessage::ContentUpd(i.into())));
        let submit_button = Button::new("Create").on_press(GuiMes::DialogSubmit);
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
