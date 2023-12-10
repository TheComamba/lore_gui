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
    pub(super) label_view_state: DbColViewState,
    pub(super) current_content: String,
}

impl HistoryViewState {
    pub(super) fn new(years: Vec<i32>) -> Self {
        let years = years.iter().map(|y| y.to_string()).collect();
        Self {
            history_items: vec![],
            year_view_state: DbColViewState::new(years),
            day_view_state: DbColViewState::default(),
            label_view_state: DbColViewState::default(),
            current_content: String::new(),
        }
    }
}

impl Default for HistoryViewState {
    fn default() -> Self {
        Self::new(vec![])
    }
}
