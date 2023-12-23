use lorecore::sql::history::HistoryItem;

use super::db_col_view::state::DbColViewState;

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
    history_items: Vec<HistoryItem>,
    pub(super) year_view_state: DbColViewState,
    pub(super) day_view_state: DbColViewState,
    pub(super) timestamp_view_state: DbColViewState,
    pub(super) current_content: String,
}

impl HistoryViewState {
    pub(super) fn new(history_items: Vec<HistoryItem>) -> Self {
        Self {
            history_items,
            year_view_state: DbColViewState::default(),
            day_view_state: DbColViewState::default(),
            timestamp_view_state: DbColViewState::default(),
            current_content: String::new(),
        }
    }

    pub(super) fn set_history_items(&mut self, history_items: Vec<HistoryItem>) {
        self.history_items = history_items;
    }

    pub(super) fn get_timestamps(&self, year: i32, day: Option<i32>) -> Vec<i64> {
        let mut timestamps: Vec<i64> = self
            .history_items
            .iter()
            .filter(|item| item.year == year)
            .filter(|item| item.day == day)
            .map(|item| item.timestamp)
            .collect();
        timestamps.sort();
        timestamps.dedup();
        timestamps
    }

    pub(super) fn get_content(&self, timestamp: i64) -> String {
        self.history_items
            .iter()
            .find(|item| item.timestamp == timestamp)
            .map(|item| item.content.clone())
            .unwrap_or(String::new())
    }
}

impl Default for HistoryViewState {
    fn default() -> Self {
        Self::new(vec![])
    }
}
