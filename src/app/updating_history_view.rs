use lorecore::sql::{lore_database::LoreDatabase, search_text::HistoryItemSearchParams};

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
        let years = self.get_years(db)?;
        self.year_view_state.set_entries(years);
        self.update_days(db)?;
        Ok(())
    }

    fn get_years(&self, db: &Option<LoreDatabase>) -> Result<Vec<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let search_int = self.year_view_state.get_search_int()?;
        let search_params = HistoryItemSearchParams::new(search_int, None);
        let history_items = db
            .get_history_items(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let years = history_items
            .iter()
            .map(|item| item.year.to_string())
            .collect::<Vec<String>>();
        Ok(years)
    }

    fn optional_int_to_string(opt: &Option<i32>) -> String {
        match opt {
            None => String::new(),
            Some(i) => i.to_string(),
        }
    }

    fn update_days(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let days = self
            .get_days(db)?
            .iter()
            .map(Self::optional_int_to_string)
            .collect::<Vec<String>>();
        self.day_view_state.set_entries(days);
        self.update_timestamps(db)?;
        Ok(())
    }

    fn get_days(&self, db: &Option<LoreDatabase>) -> Result<Vec<Option<i32>>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = match self.year_view_state.get_selected_as().unwrap_or(None) {
            Some(year) => Some(year),
            None => return Ok(vec![]),
        };

        let search_int = self.day_view_state.get_search_int()?;
        let search_params = HistoryItemSearchParams::new(year, search_int);
        let history_items = db
            .get_history_items(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let days = history_items
            .iter()
            .map(|item| item.day)
            .collect::<Vec<Option<i32>>>();
        Ok(days)
    }

    fn update_timestamps(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let timestamps = self
            .get_timestamps(db)?
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>();
        self.timestamp_view_state.set_entries(timestamps);
        self.update_content(db)?;
        Ok(())
    }

    fn get_timestamps(&self, db: &Option<LoreDatabase>) -> Result<Vec<i64>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = match self.year_view_state.get_selected_as().unwrap_or(None) {
            Some(year) => Some(year),
            None => return Ok(vec![]),
        };
        let day = self.day_view_state.get_selected_as().unwrap_or(None);

        let search_params = HistoryItemSearchParams::new(year, day);
        let history_items = db
            .get_history_items(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let timestamps = history_items
            .iter()
            .map(|item| item.timestamp)
            .collect::<Vec<i64>>();
        Ok(timestamps)
    }

    fn update_content(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        self.current_content = self.get_content(db)?;
        Ok(())
    }

    fn get_content(&self, db: &Option<LoreDatabase>) -> Result<String, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(String::new()),
        };
        let timestamp = match self.timestamp_view_state.get_selected() {
            Some(timestamp) => timestamp,
            None => return Ok(String::new()),
        };

        let search_params = HistoryItemSearchParams::new(None, None, timestamp);
        let history_items = db
            .get_history_items(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let content = history_items.iter().map(|item| item.content.clone());
        Ok(content)
    }
}
