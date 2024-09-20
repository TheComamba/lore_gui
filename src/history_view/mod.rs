use iced::widget::text_editor;
use lorecore::{
    extractions::{extract_days, extract_years},
    sql::{lore_database::LoreDatabase, search_params::HistoryItemSearchParams},
    types::{day::Day, history_item_content::HistoryItemContent, timestamp::Timestamp, year::Year},
};

use crate::{
    db_col_view::ColViewMes, dialog::redate_history::RedateHistoryData, errors::LoreGuiError,
};

use super::db_col_view::state::DbColViewState;

mod widget;

pub(super) struct HistoryView<'a> {
    state: &'a HistoryViewState,
}
pub(super) struct HistoryViewState {
    pub(super) year_view_state: DbColViewState<Year>,
    pub(super) day_view_state: DbColViewState<Day>,
    pub(super) timestamp_view_state: DbColViewState<Timestamp>,
    pub(super) current_content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub(super) enum HistoryViewMessage {
    NewHistoryItem,
    RedateHistoryItem(RedateHistoryData),
    DeleteHistoryItem(Timestamp),
    YearViewUpd(ColViewMes<Year>),
    DayViewUpd(ColViewMes<Day>),
    HistoryTimestampViewUpd(ColViewMes<Timestamp>),
}

impl HistoryViewState {
    pub(super) fn new() -> Self {
        Self {
            year_view_state: DbColViewState::default(),
            day_view_state: DbColViewState::default(),
            timestamp_view_state: DbColViewState::default(),
            current_content: text_editor::Content::with_text(""),
        }
    }

    pub(super) fn get_current_years(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Year>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = self.year_view_state.get_search_int()?.map(|y| y.into());
        let search_params = HistoryItemSearchParams::new(year, None, None, None);
        let history_items = db.read_history_items(search_params)?;
        let years = extract_years(&history_items);
        Ok(years)
    }

    pub(super) fn get_current_days(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Day>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = match self.year_view_state.get_selected().0 {
            Some(year) => Some(year),
            None => return Ok(vec![]),
        };

        let day = self.day_view_state.get_search_int()?.map(|d| d.into());
        let search_params = HistoryItemSearchParams::new(year, day, None, None);
        let history_items = db.read_history_items(search_params)?;
        Ok(extract_days(&history_items))
    }

    pub(super) fn get_current_timestamps(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Timestamp>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = match self.year_view_state.get_selected().0 {
            Some(year) => Some(year),
            None => return Ok(vec![]),
        };
        let day = self.day_view_state.get_selected().0;

        let search_params = HistoryItemSearchParams::new(year, day, None, None);
        let history_items = db.read_history_items(search_params)?;
        let timestamps = history_items
            .iter()
            .map(|item| item.timestamp)
            .collect::<Vec<Timestamp>>();
        Ok(timestamps)
    }

    pub(super) fn get_current_content(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<HistoryItemContent, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok("".into()),
        };
        let timestamp = match self.timestamp_view_state.get_selected().0 {
            Some(timestamp) => timestamp,
            None => return Ok("".into()),
        };

        let search_params = HistoryItemSearchParams::new(None, None, Some(timestamp), None);
        let history_items = db.read_history_items(search_params)?;
        if history_items.len() > 1 {
            return Err(LoreGuiError::MultipleResults);
        }
        let content = match history_items.first() {
            Some(item) => item.content.clone(),
            None => "".into(),
        };
        Ok(content)
    }
}

impl Default for HistoryViewState {
    fn default() -> Self {
        Self::new()
    }
}
