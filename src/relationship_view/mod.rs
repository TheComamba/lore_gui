use crate::{
    db_col_view::{state::DbColViewState, ColViewMes},
    errors::LoreGuiError,
};
use lorecore::sql::{
    lore_database::LoreDatabase,
    relationships::{extract_children, extract_parents},
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

#[derive(Debug, Clone)]
pub(super) enum RelationshipViewMessage {
    NewRelationship,
    ParentViewUpd(ColViewMes),
    ChildViewUpd(ColViewMes),
    RoleViewUpd(ColViewMes),
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
        let relationships = db.read_relationships(search_params)?;
        let parents = extract_parents(&relationships);
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
        let relationships = db.read_relationships(search_params)?;
        let children = extract_children(&relationships);
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
        let relationships = db.read_relationships(search_params)?;
        if relationships.len() > 1 {
            return Err(LoreGuiError::MultipleResults);
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
