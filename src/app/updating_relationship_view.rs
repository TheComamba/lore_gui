use lorecore::{
    extractions::extract_labels,
    sql::{lore_database::LoreDatabase, search_params::EntityColumnSearchParams},
    types::*,
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
        self.relationship_view_state.update(&self.lore_database)?;
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
            }
            ColViewMes::Selected(_index, parent) => {
                state.set_selected_parent(parent.0);
                state.set_selected_role(None);
            }
        };
        self.relationship_view_state.update(&self.lore_database)?;
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
            }
            ColViewMes::Selected(_index, child) => {
                state.set_selected_child(child.0);
                state.set_selected_role(None);
            }
        };
        self.relationship_view_state.update(&self.lore_database)?;
        Ok(())
    }

    pub(super) fn update_role_view(&mut self, event: ColViewMes<Role>) -> Result<(), LoreGuiError> {
        let state = &mut self.relationship_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.role_view_state.set_search_text(text);
            }
            ColViewMes::Selected(_index, role) => {
                state.set_selected_role(role.0);
            }
        };
        self.relationship_view_state.update(&self.lore_database)?;
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
        let parent = data.parent().clone();
        let child = data.child().clone();
        let role = data.role().clone();
        data.write_to_database(db)?;
        self.set_selected_parent(Some(parent));
        self.set_selected_child(Some(child));
        self.set_selected_role(Some(role));
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
        let parent = data.parent().clone().into();
        let child = data.child().clone().into();
        let role = data.new_role().clone().into();
        data.write_to_database(db)?;
        self.set_selected_parent(parent);
        self.set_selected_child(child);
        self.set_selected_role(role);
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
        self.set_selected_parent(None);
        self.set_selected_child(None);
        self.set_selected_role(None);
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

    fn update(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
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

#[cfg(test)]
mod test {
    use super::*;

    use crate::tests::{example_database, example_labels, example_role};

    #[test]
    fn selecting_parent_deselects_role() {
        let mut gui = SqlGui {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let parents = example_labels();
        let role = example_role(&parents[0].to_str().into(), &parents[1].to_str().into());
        gui.set_selected_parent(Some(parents[0].to_str().into()));
        gui.set_selected_role(Some(role.clone()));

        let new_parent: Parent = parents[2].to_str().into();
        let event = ColViewMes::Selected(1, DbColViewEntry(Some(new_parent.clone())));
        gui.update_parent_view(event).unwrap();

        assert_eq!(gui.get_selected_parent(), Some(new_parent));
        assert_eq!(gui.get_selected_role(), None);
    }

    #[test]
    fn selecting_child_deselects_role() {
        let mut gui = SqlGui {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let children = example_labels();
        let role = example_role(&children[0].to_str().into(), &children[1].to_str().into());
        gui.set_selected_child(Some(children[0].to_str().into()));
        gui.set_selected_role(Some(role.clone()));

        let new_child: Child = children[2].to_str().into();
        let event = ColViewMes::Selected(1, DbColViewEntry(Some(new_child.clone())));
        gui.update_child_view(event).unwrap();

        assert_eq!(gui.get_selected_child(), Some(new_child));
        assert_eq!(gui.get_selected_role(), None);
    }
}
