use lorecore::sql::lore_database::LoreDatabase;

use crate::{
    db_col_view::ColViewMes,
    dialog::new_history_item::{NewHistoryData, NewHistoryDialog},
    errors::LoreGuiError,
    history_view::HistoryViewState,
};

use super::SqlGui;

impl SqlGui {
    pub(super) fn update_year_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::New => self.dialog = Some(Box::new(NewHistoryDialog::new())),
            ColViewMes::SearchFieldUpd(text) => {
                state.year_view_state.set_search_text(text);
                state.update_years(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, year) => {
                state.year_view_state.set_selected(year);
                state.day_view_state.set_selected_none();
                state.update_days(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_day_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.day_view_state.set_search_text(text);
                state.update_days(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, day) => {
                state.day_view_state.set_selected(day);
                state.timestamp_view_state.set_selected_none();
                state.update_timestamps(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_timestamp_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.timestamp_view_state.set_search_text(text);
                state.update_timestamps(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, timestamp) => {
                state.timestamp_view_state.set_selected(timestamp);
                state.update_content(&self.lore_database)?;
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
    pub(super) fn reset_selections(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<(), LoreGuiError> {
        self.year_view_state.set_selected_none();
        self.day_view_state.set_selected_none();
        self.timestamp_view_state.set_selected_none();
        self.current_content = String::new();
        self.update_years(db)?;
        Ok(())
    }

    fn update_years(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let years = self
            .get_current_years(db)?
            .iter()
            .map(|y| y.to_string())
            .collect();
        self.year_view_state.set_entries(years);
        self.update_days(db)?;
        Ok(())
    }

    fn update_days(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let days = self
            .get_current_days(db)?
            .iter()
            .map(Self::optional_int_to_string)
            .collect::<Vec<String>>();
        self.day_view_state.set_entries(days);
        self.update_timestamps(db)?;
        Ok(())
    }

    fn update_timestamps(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let timestamps = self
            .get_current_timestamps(db)?
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>();
        self.timestamp_view_state.set_entries(timestamps);
        self.update_content(db)?;
        Ok(())
    }

    fn update_content(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        self.current_content = self.get_current_content(db)?;
        Ok(())
    }

    fn optional_int_to_string(opt: &Option<i32>) -> String {
        match opt {
            None => String::new(),
            Some(i) => i.to_string(),
        }
    }
}
