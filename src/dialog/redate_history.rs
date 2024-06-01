use iced::{
    widget::{component, Button, Column, Component, Text, TextInput},
    Element,
};
use lorecore::{
    errors::LoreCoreError,
    sql::lore_database::LoreDatabase,
    types::{day::Day, timestamp::Timestamp, year::Year},
};

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::Dialog;

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
            new_day: old_day.clone(),
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
            self.data.old_day.to_string(),
            self.data.timestamp
        )
    }

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }
}

impl Component<GuiMes> for RedateHistoryDialog {
    type State = ();

    type Event = RedateHistoryMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            RedateHistoryMes::YearUpd(year) => {
                if let Ok(year) = year {
                    self.data.new_year = year;
                }
                None
            }
            RedateHistoryMes::DayUpd(day) => {
                if let Ok(day) = day {
                    self.data.new_day = day;
                }
                None
            }
            RedateHistoryMes::Submit => Some(GuiMes::RedateHistoryItem(self.data.to_owned())),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let year_input = TextInput::new("", &self.data.new_year.to_string())
            .on_input(|i| RedateHistoryMes::YearUpd(i.try_into()));
        let day_input = TextInput::new("", &self.data.new_day.to_string())
            .on_input(|i| RedateHistoryMes::DayUpd(i.try_into()));
        let submit_button = Button::new("Redate").on_press(RedateHistoryMes::Submit);
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
}

#[derive(Debug, Clone)]
pub(crate) enum RedateHistoryMes {
    YearUpd(Result<Year, LoreCoreError>),
    DayUpd(Result<Day, LoreCoreError>),
    Submit,
}
