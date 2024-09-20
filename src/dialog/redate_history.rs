use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{day::Day, timestamp::Timestamp, year::Year},
};

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::{Dialog, DialogMessage};

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
}

impl Dialog for RedateHistoryDialog {
    fn header(&self) -> String {
        format!(
            "Redate history for entity: year {}, day {} ({})",
            self.data.old_year,
            self.data.old_day,
            self.data.timestamp
        )
    }

    fn body(&self) -> Element<'_, GuiMes> {
        let year_input = TextInput::new("", &self.data.new_year.to_string())
            .on_input(|i| GuiMes::DialogUpdate(DialogMessage::YearUpd(i.try_into())));
        let day_input = TextInput::new("", &self.data.new_day.to_string())
            .on_input(|i| GuiMes::DialogUpdate(DialogMessage::DayUpd(i.try_into())));
        let submit_button = Button::new("Redate").on_press(GuiMes::DialogSubmit);
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

    fn update(&mut self, message: DialogMessage) {
        match message {
            DialogMessage::YearUpd(year) => {
                if let Ok(year) = year {
                    self.data.new_year = year;
                }
            }
            DialogMessage::DayUpd(day) => {
                if let Ok(day) = day {
                    self.data.new_day = day;
                }
            }
            _ => (),
        }
    }

    fn submit(&self) -> GuiMes {
        GuiMes::RedateHistoryItem(self.data.to_owned())
    }
}
