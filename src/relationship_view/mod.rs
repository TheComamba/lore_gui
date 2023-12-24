use crate::{db_col_view::state::DbColViewState, errors::LoreGuiError};
use lorecore::sql::{
    lore_database::LoreDatabase,
    search_params::{RelationshipSearchParams, SqlSearchText},
};

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
    pub(super) parent_view_state: DbColViewState,
    pub(super) child_view_state: DbColViewState,
    pub(super) current_role: Option<String>,
}

impl RelationshipViewState {
    pub(super) fn new() -> Self {
        Self {
            parent_view_state: DbColViewState::default(),
            child_view_state: DbColViewState::default(),
            current_role: None,
        }
    }

    pub(super) fn get_current_parents(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let child = self
            .child_view_state
            .get_selected()
            .as_ref()
            .map(|c| SqlSearchText::exact(c.as_str()));
        let parent_search_text = self
            .parent_view_state
            .get_search_text()
            .map(|t| SqlSearchText::partial(t));
        let search_params = RelationshipSearchParams::new(parent_search_text, child);
        let relationships = db
            .get_relationships(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let parents = relationships.iter().map(|rel| rel.parent.clone()).collect();
        Ok(parents)
    }

    pub(super) fn get_current_children(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let parent = self
            .parent_view_state
            .get_selected()
            .as_ref()
            .map(|p| SqlSearchText::exact(p.as_str()));
        let child_search_text = self
            .child_view_state
            .get_search_text()
            .map(|t| SqlSearchText::partial(t));
        let search_params = RelationshipSearchParams::new(parent, child_search_text);
        let relationships = db
            .get_relationships(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let children = relationships.iter().map(|rel| rel.child.clone()).collect();
        Ok(children)
    }

    pub(super) fn get_current_role(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Option<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(None),
        };
        let parent = match self.parent_view_state.get_selected() {
            Some(parent) => parent,
            None => return Ok(None),
        };
        let child = match self.child_view_state.get_selected() {
            Some(child) => child,
            None => return Ok(None),
        };
        let search_params = RelationshipSearchParams::new(
            Some(SqlSearchText::exact(parent.as_str())),
            Some(SqlSearchText::exact(child.as_str())),
        );
        let relationships = db
            .get_relationships(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        if relationships.len() > 1 {
            return Err(LoreGuiError::InputError(
                "Multiple relationships found".to_string(),
            ));
        }
        let role = match relationships.first() {
            Some(relationship) => relationship.role.clone(),
            None => None,
        };
        Ok(role)
    }
}

impl Default for RelationshipViewState {
    fn default() -> Self {
        Self::new()
    }
}
