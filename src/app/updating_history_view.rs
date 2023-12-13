use crate::{
    db_col_view::{state::DbColViewState, ColViewMes},
    dialog::new_history_item::{NewHistoryData, NewHistoryDialog},
    errors::LoreGuiError,
    history_view::HistoryViewState,
};
use lorecore::sql::lore_database::LoreDatabase;

use super::SqlGui;

impl SqlGui {
    pub(super) fn update_year_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::New => self.dialog = Some(Box::new(NewHistoryDialog::new())),
            ColViewMes::SearchFieldUpd(text) => {
                state.year_view_state.set_search_text(text);
                state.update_years(db)?;
            }
            ColViewMes::Selected(_index, year) => {
                state.year_view_state.set_selected(year);
                state.day_view_state.set_selected_none();
                state.update_days(db)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_day_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.day_view_state.set_search_text(text);
                state.update_days(db)?;
            }
            ColViewMes::Selected(_index, day) => {
                state.day_view_state.set_selected(day);
                state.timestamp_view_state.set_selected_none();
                state.update_timestamps(db)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_history_timestamp_view(
        &mut self,
        event: ColViewMes,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.timestamp_view_state.set_search_text(text);
                state.update_timestamps(db)?;
            }
            ColViewMes::Selected(_index, timestamp) => {
                state.timestamp_view_state.set_selected(timestamp);
                state.update_content(db)?;
            }
        };
        Ok(())
    }

    pub(super) fn write_new_history(&mut self, data: NewHistoryData) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let year = data.year.to_string();
        let day = match data.day {
            Some(day) => day.to_string(),
            None => String::new(),
        };
        data.write_to_database(db)?;
        self.update_year_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_year_view(ColViewMes::Selected(0, year))?;
        self.update_day_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_day_view(ColViewMes::Selected(0, day))?;
        self.dialog = None;
        Ok(())
    }
}

impl HistoryViewState {
    pub(super) fn reset(&mut self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        self.reset_selections();
        self.update_years(db)?;
        Ok(())
    }

    fn reset_selections(&mut self) {
        self.year_view_state.set_selected_none();
        self.day_view_state.set_selected_none();
        self.timestamp_view_state.set_selected_none();
        self.current_content = String::new();
    }

    fn update_years(&mut self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        let years = self.get_all_years().iter().map(|y| y.to_string()).collect();
        self.year_view_state.set_entries(years);
        self.update_days(db)?;
        Ok(())
    }

    fn optional_int_to_string(opt: &Option<i32>) -> String {
        match opt {
            None => String::new(),
            Some(i) => i.to_string(),
        }
    }

    fn update_days(&mut self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        let year = self.year_view_state.get_selected_as()?;
        match year {
            Some(year) => {
                let days = self
                    .get_days(year)
                    .iter()
                    .map(Self::optional_int_to_string)
                    .collect();
                self.day_view_state.set_entries(days);
            }
            None => {
                self.day_view_state = DbColViewState::default();
            }
        }
        self.update_timestamps(db)?;
        Ok(())
    }

    fn update_timestamps(&mut self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        let year = self.year_view_state.get_selected_as()?;
        let day = self.day_view_state.get_selected_as()?;
        match year {
            Some(year) => {
                self.timestamp_view_state.set_entries(
                    self.get_timestamps(year, day)
                        .iter()
                        .map(|t| t.to_string())
                        .collect(),
                );
            }
            None => (),
        }
        self.update_content(db)?;
        Ok(())
    }

    fn update_content(&mut self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        let timestamp = match self.timestamp_view_state.get_selected_as()? {
            Some(timestamp) => timestamp,
            None => {
                self.current_content = "".to_string();
                return Ok(());
            }
        };
        self.current_content = self.get_content(timestamp);
        Ok(())
    }
}
