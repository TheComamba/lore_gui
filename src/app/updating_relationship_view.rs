use lorecore::{
    extractions::extract_labels,
    sql::{lore_database::LoreDatabase, search_params::EntityColumnSearchParams},
    types::{
        child::Child, label::Label, parent::Parent, relationship::EntityRelationship, role::Role,
    },
};

use crate::{
    db_col_view::{entry::DbColViewEntry, ColViewMes},
    dialog::{
        change_role::{ChangeRoleData, ChangeRoleDialog},
        confirmation::ConfirmationDialog,
        new_relationship::{NewRelationshipData, NewRelationshipDialog},
    },
    errors::LoreGuiError,
    relationship_view::{RelationshipViewMessage, RelationshipViewState},
};

use super::{message_handling::GuiMessage, SqlGui};

impl SqlGui {
    pub(super) fn update_relationship_view(
        &mut self,
        event: RelationshipViewMessage,
    ) -> Result<(), LoreGuiError> {
        match event {
            RelationshipViewMessage::NewRelationship => {
                let labels = self.get_all_labels(&self.lore_database)?;
                self.dialog = Some(Box::new(NewRelationshipDialog::new(
                    labels.clone().into_iter().map(|l| l.into()).collect(),
                    labels.clone().into_iter().map(|l| l.into()).collect(),
                )));
            }
            RelationshipViewMessage::ChangeRole(data) => {
                self.dialog = Some(Box::new(ChangeRoleDialog::new(data.clone())));
            }
            RelationshipViewMessage::DeleteRelationship(rel) => {
                let message = format!(
                    "Do you really want to delete the {} relationship between {} and {}?",
                    rel.role, rel.parent, rel.child
                );
                let on_confirm = GuiMessage::DeleteRelationship(rel);
                self.dialog = Some(Box::new(ConfirmationDialog::new(message, on_confirm)))
            }
            RelationshipViewMessage::ParentViewUpdate(event) => {
                self.update_parent_view(event)?;
            }
            RelationshipViewMessage::ChildViewUpdate(event) => {
                self.update_child_view(event)?;
            }
            RelationshipViewMessage::RoleViewUpdate(event) => {
                self.update_role_view(event)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_parent_view(
        &mut self,
        event: ColViewMes<Parent>,
    ) -> Result<(), LoreGuiError> {
        let state = &mut self.relationship_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.parent_view_state.set_search_text(text);
                state.update_parents(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, parent) => {
                state.parent_view_state.set_selected(parent);
                state.update_children(&self.lore_database)?;
                state.update_role(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_child_view(
        &mut self,
        event: ColViewMes<Child>,
    ) -> Result<(), LoreGuiError> {
        let state = &mut self.relationship_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.child_view_state.set_search_text(text);
                state.update_children(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, child) => {
                state.child_view_state.set_selected(child);
                state.update_parents(&self.lore_database)?;
                state.update_role(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_role_view(&mut self, event: ColViewMes<Role>) -> Result<(), LoreGuiError> {
        let state = &mut self.relationship_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.role_view_state.set_search_text(text);
            }
            ColViewMes::Selected(_index, role) => {
                state.role_view_state.set_selected(role);
            }
        };
        Ok(())
    }

    pub(super) fn write_new_relationship(
        &mut self,
        data: NewRelationshipData,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        data.write_to_database(db)?;
        self.update_parent_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_child_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.dialog = None;
        Ok(())
    }

    pub(super) fn change_relationship_role(
        &mut self,
        data: ChangeRoleData,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        data.write_to_database(db)?;
        self.update_role_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.dialog = None;
        Ok(())
    }

    pub(super) fn delete_relationship(
        &mut self,
        rel: EntityRelationship,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        db.delete_relationship(rel)?;
        self.update_parent_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_child_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.dialog = None;
        Ok(())
    }

    fn get_all_labels(&self, db: &Option<LoreDatabase>) -> Result<Vec<Label>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let search_params = EntityColumnSearchParams::new(None, None);
        let entity_columns = db.read_entity_columns(search_params)?;
        let labels = extract_labels(&entity_columns);
        Ok(labels)
    }
}

impl RelationshipViewState {
    pub(super) fn reset_selections(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<(), LoreGuiError> {
        self.parent_view_state.set_selected(DbColViewEntry::NONE);
        self.child_view_state.set_selected(DbColViewEntry::NONE);
        self.role_view_state.set_selected(DbColViewEntry::NONE);
        self.update_parents(db)?;
        self.update_children(db)?;
        self.update_role(db)?;
        Ok(())
    }

    fn update_parents(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let parents = self
            .get_current_parents(db)?
            .into_iter()
            .map(|p| DbColViewEntry(Some(p)))
            .collect();
        self.parent_view_state.set_entries(parents);
        Ok(())
    }

    fn update_children(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let children = self
            .get_current_children(db)?
            .into_iter()
            .map(|c| DbColViewEntry(Some(c)))
            .collect();
        self.child_view_state.set_entries(children);
        Ok(())
    }

    fn update_role(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let roles = self
            .get_current_roles(db)?
            .into_iter()
            .map(|r| DbColViewEntry(Some(r)))
            .collect();
        self.role_view_state.set_entries(roles);
        Ok(())
    }
}
