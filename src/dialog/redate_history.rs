use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{sql::lore_database::LoreDatabase, types::*};

use crate::{app::message_handling::GuiMessage, errors::LoreGuiError};

use super::{Dialog, DialogUpdate};

#[derive(Debug, Clone)]
pub(crate) struct RedateHistoryDialog {
    data: RedateHistoryData,
}

impl RedateHistoryDialog {
    pub(crate) fn new(data: RedateHistoryData) -> Self {
        RedateHistoryDialog { data }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RedateHistoryData {
    pub(self) timestamp: Timestamp,
    pub(self) old_year: Year,
    pub(self) old_day: Day,
    pub(self) new_year: Year,
    pub(self) new_day: Day,
}

impl RedateHistoryData {
    pub(crate) fn new(timestamp: Timestamp, old_year: Year, old_day: Day) -> Self {
        RedateHistoryData {
            timestamp,
            new_year: old_year,
            new_day: old_day,
            old_year,
            old_day,
        }
    }

    pub(crate) fn update_date_in_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        db.redate_history_item(self.timestamp, self.new_year, self.new_day)?;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn set_new_year(&mut self, year: Year) {
        self.new_year = year;
    }

    #[cfg(test)]
    pub(crate) fn set_new_day(&mut self, day: Day) {
        self.new_day = day;
    }
}

impl Dialog for RedateHistoryDialog {
    fn header(&self) -> String {
        format!(
            "Redate history for entity: year {}, day {} ({})",
            self.data.old_year, self.data.old_day, self.data.timestamp
        )
    }

    fn body(&self) -> Element<'_, GuiMessage> {
        let year_input = TextInput::new("", &self.data.new_year.to_string())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Year(i.try_into())));
        let day_input = TextInput::new("", &self.data.new_day.to_string())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Day(i.try_into())));
        let submit_button = Button::new("Redate").on_press(GuiMessage::DialogSubmit);
        Column::new()
            .push(Text::new("Year:"))
            .push(year_input)
            .push(Text::new("Day (optional):"))
            .push(day_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn update(&mut self, message: DialogUpdate) {
        match message {
            DialogUpdate::Year(Ok(year)) => self.data.new_year = year,
            DialogUpdate::Day(Ok(day)) => self.data.new_day = day,
            _ => (),
        }
    }

    fn submit(&self) -> GuiMessage {
        GuiMessage::RedateHistoryItem(self.data.to_owned())
    }
}
