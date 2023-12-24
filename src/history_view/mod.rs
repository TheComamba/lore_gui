use super::db_col_view::state::DbColViewState;
use crate::errors::LoreGuiError;
use lorecore::sql::{lore_database::LoreDatabase, search_params::HistoryItemSearchParams};

mod widget;

pub(super) struct HistoryView<'a> {
    state: &'a HistoryViewState,
}

impl<'a> HistoryView<'a> {
    pub(super) fn new(state: &'a HistoryViewState) -> Self {
        Self { state }
    }
}

pub(super) struct HistoryViewState {
    pub(super) year_view_state: DbColViewState,
    pub(super) day_view_state: DbColViewState,
    pub(super) timestamp_view_state: DbColViewState,
    pub(super) current_content: String,
}

impl HistoryViewState {
    pub(super) fn new() -> Self {
        Self {
            year_view_state: DbColViewState::default(),
            day_view_state: DbColViewState::default(),
            timestamp_view_state: DbColViewState::default(),
            current_content: String::new(),
        }
    }

    pub(super) fn get_current_years(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = self.year_view_state.get_search_int()?;
        let search_params = HistoryItemSearchParams::new(year, None, None);
        let history_items = db
            .get_history_items(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let years = history_items
            .iter()
            .map(|item| item.year.to_string())
            .collect::<Vec<String>>();
        Ok(years)
    }

    pub(super) fn get_current_days(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Option<i32>>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = match self.year_view_state.get_selected_as().unwrap_or(None) {
            Some(year) => Some(year),
            None => return Ok(vec![]),
        };

        let day = self.day_view_state.get_search_int()?;
        let search_params = HistoryItemSearchParams::new(year, day, None);
        let history_items = db
            .get_history_items(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let days = history_items
            .iter()
            .map(|item| item.day)
            .collect::<Vec<Option<i32>>>();
        Ok(days)
    }

    pub(super) fn get_current_timestamps(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<i64>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = match self.year_view_state.get_selected_as().unwrap_or(None) {
            Some(year) => Some(year),
            None => return Ok(vec![]),
        };
        let day = self.day_view_state.get_selected_as().unwrap_or(None);

        let search_params = HistoryItemSearchParams::new(year, day, None);
        let history_items = db
            .get_history_items(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let timestamps = history_items
            .iter()
            .map(|item| item.timestamp)
            .collect::<Vec<i64>>();
        Ok(timestamps)
    }

    pub(super) fn get_current_content(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<String, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(String::new()),
        };
        let timestamp = match self.timestamp_view_state.get_selected_as().unwrap_or(None) {
            Some(timestamp) => timestamp,
            None => return Ok(String::new()),
        };

        let search_params = HistoryItemSearchParams::new(None, None, Some(timestamp));
        let history_items = db
            .get_history_items(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        if history_items.len() > 1 {
            return Err(LoreGuiError::InputError(
                "Multiple history items found".to_string(),
            ));
        }
        let content = match history_items.first() {
            Some(item) => item.content.clone(),
            None => String::new(),
        };
        Ok(content)
    }
}

impl Default for HistoryViewState {
    fn default() -> Self {
        Self::new()
    }
}
