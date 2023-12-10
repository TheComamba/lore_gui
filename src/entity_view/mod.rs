use lorecore::sql::entity::EntityColumn;

use super::db_col_view::state::DbColViewState;

mod widget;

pub(super) struct EntityView<'a> {
    state: &'a EntityViewState,
}

pub(super) struct EntityViewState {
    entity_columns: Vec<EntityColumn>,
    pub(super) label_view_state: DbColViewState,
    pub(super) descriptor_view_state: DbColViewState,
    pub(super) current_description: Option<String>,
}

impl<'a> EntityView<'a> {
    pub(super) fn new(state: &'a EntityViewState) -> Self {
        Self { state }
    }
}

impl EntityViewState {
    pub(super) fn new(labels: Vec<String>) -> Self {
        Self {
            entity_columns: vec![],
            label_view_state: DbColViewState::new(labels),
            descriptor_view_state: DbColViewState::default(),
            current_description: None,
        }
    }
}

impl Default for EntityViewState {
    fn default() -> Self {
        Self::new(vec![])
    }
}
