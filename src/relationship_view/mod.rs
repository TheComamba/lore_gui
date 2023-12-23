use lorecore::sql::relationships::EntityRelationship;

use crate::db_col_view::state::DbColViewState;

mod widget;

pub(super) struct RelationshipView<'a> {
    state: &'a RelationshipViewState,
}

impl<'a> RelationshipView<'a> {
    pub(super) fn new(state: &'a RelationshipViewState) -> Self {
        Self { state }
    }
}

pub(super) struct RelationshipViewState {
    relationships: Vec<EntityRelationship>,
    pub(super) parent_view_state: DbColViewState,
    pub(super) child_view_state: DbColViewState,
    pub(super) current_role: Option<String>,
}

impl RelationshipViewState {
    pub(super) fn new(relationships: Vec<EntityRelationship>) -> Self {
        Self {
            relationships,
            parent_view_state: DbColViewState::default(),
            child_view_state: DbColViewState::default(),
            current_role: None,
        }
    }

    pub(super) fn set_relationships(&mut self, relationships: Vec<EntityRelationship>) {
        self.relationships = relationships;
    }
}

impl Default for RelationshipViewState {
    fn default() -> Self {
        Self::new(vec![])
    }
}
