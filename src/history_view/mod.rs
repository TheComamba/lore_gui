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
    pub(super) year_view_state: DbColViewState,
    pub(super) day_view_state: DbColViewState,
    pub(super) timestamp_view_state: DbColViewState,
    pub(super) current_content: String,
}

impl HistoryViewState {
    pub(super) fn new(history_items: Vec<HistoryItem>) -> Self {
        Self {
            year_view_state: DbColViewState::default(),
            day_view_state: DbColViewState::default(),
            timestamp_view_state: DbColViewState::default(),
            current_content: String::new(),
        }
    }
}

impl Default for HistoryViewState {
    fn default() -> Self {
        Self::new(vec![])
    }
}
