use lorecore::{
    sql::{
        lore_database::LoreDatabase,
        relationship::{extract_children, extract_parents, extract_roles},
        search_params::{RelationshipSearchParams, SqlSearchText},
    },
    types::*,
};

use crate::{
    db_col_view::{entry::DbColViewEntry, state::DbColViewState, ColViewMes},
    dialog::change_role::ChangeRoleData,
    errors::LoreGuiError,
};

pub(crate) mod widget;

pub(super) struct RelationshipViewState {
    pub(super) parent_view_state: DbColViewState<Parent>,
    pub(super) child_view_state: DbColViewState<Child>,
    pub(super) role_view_state: DbColViewState<Role>,
}

#[derive(Debug, Clone)]
pub(super) enum RelationshipViewMessage {
    NewRelationship,
    ChangeRole(ChangeRoleData),
    DeleteRelationship(EntityRelationship),
    ParentViewUpdate(ColViewMes<Parent>),
    ChildViewUpdate(ColViewMes<Child>),
    RoleViewUpdate(ColViewMes<Role>),
}

impl RelationshipViewState {
    pub(super) fn new() -> Self {
        Self {
            parent_view_state: DbColViewState::new(vec![], true),
            child_view_state: DbColViewState::new(vec![], true),
            role_view_state: DbColViewState::default(),
        }
    }

    pub(super) fn get_current_parents(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Parent>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let child = self
            .get_selected_child()
            .map(|c| SqlSearchText::exact(c.to_str()));
        let parent_search_text = self
            .parent_view_state
            .get_search_text()
            .map(SqlSearchText::partial);
        let search_params = RelationshipSearchParams::new(parent_search_text, child);
        let relationships = db.read_relationships(search_params)?;
        let parents = extract_parents(&relationships);
        Ok(parents)
    }

    pub(super) fn get_current_children(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Child>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let parent = self
            .get_selected_parent()
            .map(|p| SqlSearchText::exact(p.to_str()));
        let child_search_text = self
            .child_view_state
            .get_search_text()
            .map(SqlSearchText::partial);
        let search_params = RelationshipSearchParams::new(parent, child_search_text);
        let relationships = db.read_relationships(search_params)?;
        let children = extract_children(&relationships);
        Ok(children)
    }

    pub(super) fn get_current_roles(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Role>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let parent = match self.get_selected_parent() {
            Some(parent) => parent,
            None => return Ok(vec![]),
        };
        let child = match self.get_selected_child() {
            Some(child) => child,
            None => return Ok(vec![]),
        };
        let search_params = RelationshipSearchParams::new(
            Some(SqlSearchText::exact(parent.to_str())),
            Some(SqlSearchText::exact(child.to_str())),
        );
        let relationships = db.read_relationships(search_params)?;
        let roles = extract_roles(&relationships);
        Ok(roles)
    }

    pub(super) fn get_selected_parent(&self) -> Option<Parent> {
        self.parent_view_state.get_selected().0.clone()
    }

    pub(super) fn set_selected_parent(&mut self, parent: Option<Parent>) {
        self.parent_view_state.set_selected(DbColViewEntry(parent));
    }

    pub(super) fn get_selected_child(&self) -> Option<Child> {
        self.child_view_state.get_selected().0.clone()
    }

    pub(super) fn set_selected_child(&mut self, child: Option<Child>) {
        self.child_view_state.set_selected(DbColViewEntry(child));
    }

    pub(super) fn get_selected_role(&self) -> Option<Role> {
        self.role_view_state.get_selected().0.clone()
    }

    pub(super) fn set_selected_role(&mut self, role: Option<Role>) {
        self.role_view_state.set_selected(DbColViewEntry(role));
    }
}

impl Default for RelationshipViewState {
    fn default() -> Self {
        Self::new()
    }
}
