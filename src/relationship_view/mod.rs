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

    pub(super) fn get_children(
        &self,
        parent: Option<&str>,
        search_text: Option<&str>,
    ) -> Vec<String> {
        let mut children: Vec<String> = self
            .relationships
            .iter()
            .filter(|e| match parent {
                Some(parent) => e.parent == parent,
                None => true,
            })
            .filter(|e| match search_text {
                Some(ref search_text) => e.child.contains(search_text),
                None => true,
            })
            .map(|rel| rel.child.clone())
            .collect();
        children.sort();
        children.dedup();
        children
    }

    pub(super) fn get_role(&self, parent: &str, child: &str) -> Option<String> {
        self.relationships
            .iter()
            .find(|e| e.parent == parent && e.child == child)
            .map(|rel| rel.role.clone())
            .unwrap_or(None)
    }
}

impl Default for RelationshipViewState {
    fn default() -> Self {
        Self::new(vec![])
    }
}
