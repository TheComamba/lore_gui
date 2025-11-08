use lorecore::types::*;

use crate::{
    app::state::{GuiState, ViewType},
    dialog::{
        change_role::ChangeRoleData, new_descriptor::NewDescriptorData, new_entity::NewEntityData,
        new_history_item::NewHistoryData, new_relationship::NewRelationshipData,
        redate_history::RedateHistoryData, relabel_entity::RelabelEntityData,
        rename_descriptor::RenameDescriptorData, DialogUpdate,
    },
    entity_view::EntityViewMessage,
    errors::LoreGuiError,
    history_view::HistoryViewMessage,
    relationship_view::RelationshipViewMessage,
};

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    ViewSelected(ViewType),
    NewDatabase,
    OpenDatabase,
    SetDisplayProtected(bool),
    EntityViewUpd(EntityViewMessage),
    HistoryViewUpd(HistoryViewMessage),
    RelationshipViewUpd(RelationshipViewMessage),
    DialogUpdate(DialogUpdate),
    DialogSubmit,
    DialogClosed,
    NewEntity(NewEntityData),
    RelabelEntity(RelabelEntityData),
    DeleteEntity(Label),
    NewDescriptor(NewDescriptorData),
    RenameDescriptor(RenameDescriptorData),
    DeleteDescriptor(Label, Descriptor),
    NewHistoryItem(NewHistoryData),
    RedateHistoryItem(RedateHistoryData),
    DeleteHistoryItem(Timestamp),
    NewRelationship(NewRelationshipData),
    ChangeRole(ChangeRoleData),
    DeleteRelationship(EntityRelationship),
}

impl GuiState {
    pub(super) fn handle_message(&mut self, message: GuiMessage) -> Result<(), LoreGuiError> {
        match message {
            GuiMessage::ViewSelected(view) => self.selected_view = view,
            GuiMessage::NewDatabase => self.new_database_from_dialog()?,
            GuiMessage::OpenDatabase => self.open_database_from_dialog()?,
            GuiMessage::SetDisplayProtected(display_protected) => {
                self.display_protected = display_protected
            }
            GuiMessage::EntityViewUpd(event) => self.update_entity_view(event)?,
            GuiMessage::HistoryViewUpd(event) => self.update_history_view(event)?,
            GuiMessage::RelationshipViewUpd(event) => self.update_relationship_view(event)?,
            GuiMessage::DialogUpdate(update) => self.update_dialog(update),
            GuiMessage::DialogSubmit => self.dialog_submit()?,
            GuiMessage::DialogClosed => self.dialog = None,
            GuiMessage::NewEntity(data) => self.write_new_entity(data)?,
            GuiMessage::RelabelEntity(data) => self.relabel_entity(data)?,
            GuiMessage::DeleteEntity(label) => self.delete_entity(label)?,
            GuiMessage::NewDescriptor(data) => self.write_new_descriptor(data)?,
            GuiMessage::RenameDescriptor(data) => self.change_descriptor(data)?,
            GuiMessage::DeleteDescriptor(label, descriptor) => {
                self.delete_descriptor(label, descriptor)?
            }
            GuiMessage::NewHistoryItem(data) => self.write_new_history(data)?,
            GuiMessage::RedateHistoryItem(data) => self.redate_history_item(data)?,
            GuiMessage::DeleteHistoryItem(timestamp) => self.delete_history_item(timestamp)?,
            GuiMessage::NewRelationship(data) => self.write_new_relationship(data)?,
            GuiMessage::ChangeRole(data) => self.change_relationship_role(data)?,
            GuiMessage::DeleteRelationship(relationship) => {
                self.delete_relationship(relationship)?
            }
        }
        self.entity_view_state.update(&self.lore_database)?;
        self.history_view_state.update(&self.lore_database)?;
        self.relationship_view_state.update(&self.lore_database)?;
        Ok(())
    }

    fn update_dialog(&mut self, update: DialogUpdate) {
        if let Some(dialog) = self.dialog.as_mut() {
            dialog.update(update);
        }
    }

    fn dialog_submit(&mut self) -> Result<(), LoreGuiError> {
        if let Some(dialog) = self.dialog.as_ref() {
            self.handle_message(dialog.submit())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dialog::new_descriptor::tests::example_new_descriptor_data;
    use crate::dialog::new_entity::tests::example_new_entity_data;
    use crate::dialog::new_history_item::tests::example_new_history_data;
    use crate::dialog::new_relationship::tests::example_new_relationship_data;
    use crate::dialog::relabel_entity::tests::example_relabel_entity_data;
    use crate::dialog::rename_descriptor::tests::example_rename_descriptor_data;
    use crate::tests::example_database;

    use super::*;

    #[test]
    fn view_selected_message_selects_a_view() {
        let mut gui = GuiState {
            selected_view: ViewType::Entity,
            ..Default::default()
        };
        let message = GuiMessage::ViewSelected(ViewType::History);
        gui.handle_message(message).unwrap();
        assert_eq!(gui.selected_view, ViewType::History);
    }

    #[test]
    fn dialog_submit_closes_dialog() {
        let dialog = Box::new(crate::dialog::error::ErrorDialog::new(
            LoreGuiError::NoDatabase,
        ));
        let mut gui = GuiState {
            dialog: Some(dialog),
            ..Default::default()
        };
        let message = GuiMessage::DialogSubmit;
        gui.handle_message(message).unwrap();
        assert!(gui.dialog.is_none());
    }

    #[test]
    fn dialog_closed_closes_dialog() {
        let mut gui = GuiState {
            dialog: Some(Box::new(crate::dialog::new_entity::NewEntityDialog::new())),
            ..Default::default()
        };
        let message = GuiMessage::DialogClosed;
        gui.handle_message(message).unwrap();
        assert!(gui.dialog.is_none());
    }

    #[test]
    fn new_entity_sets_it_selected() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let data = example_new_entity_data();
        let message = GuiMessage::NewEntity(data.clone());
        gui.handle_message(message).unwrap();
        assert_eq!(gui.get_selected_label(), Some(data.label().to_owned()));
    }

    #[test]
    fn creating_entity_that_already_exists_produces_error() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let data = example_new_entity_data();
        let message = GuiMessage::NewEntity(data.clone());
        gui.handle_message(message).unwrap();
        let message = GuiMessage::NewEntity(data.clone());
        let result = gui.handle_message(message);
        assert!(result.is_err());
    }

    #[test]
    fn relabel_entity_sets_it_selected() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let relabel_data = example_relabel_entity_data();
        let mut create_data = NewEntityData::new();
        let old_label = relabel_data.old_label().clone();
        let new_label = relabel_data.new_label().clone();
        create_data.set_label(old_label);
        let relabel_message = GuiMessage::RelabelEntity(relabel_data);
        gui.handle_message(relabel_message).unwrap();
        assert_eq!(gui.get_selected_label(), Some(new_label));
    }

    #[test]
    fn delete_entity_deselects_it() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let new_entity_data = example_new_entity_data();
        let create_message = GuiMessage::NewEntity(new_entity_data.clone());
        gui.handle_message(create_message).unwrap();
        let delete_message = GuiMessage::DeleteEntity(new_entity_data.label().to_owned());
        gui.handle_message(delete_message).unwrap();

        assert_eq!(gui.get_selected_label(), None);
        assert_eq!(gui.get_selected_descriptor(), None);
        assert_eq!(gui.get_description_text(), "\n");
    }

    #[test]
    fn new_descriptor_sets_it_selected() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let data = example_new_descriptor_data();
        let label = data.label().clone();
        let descriptor = data.descriptor().clone();
        let description = data.description().to_string();
        let message = GuiMessage::NewDescriptor(data);
        gui.handle_message(message).unwrap();

        assert_eq!(gui.get_selected_label(), Some(label));
        assert_eq!(gui.get_selected_descriptor(), Some(descriptor));
        assert_eq!(gui.get_description_text(), description);
    }

    #[test]
    fn creating_descriptor_that_already_exists_produces_error() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let data = example_new_descriptor_data();
        let message = GuiMessage::NewDescriptor(data.clone());
        gui.handle_message(message).unwrap();
        let message = GuiMessage::NewDescriptor(data.clone());
        let result = gui.handle_message(message);
        assert!(result.is_err());
    }

    #[test]
    fn rename_descriptor_selects_descriptor() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let rename_data = example_rename_descriptor_data();
        let label = rename_data.label().clone();
        let old_descriptor = rename_data.old_descriptor().clone();
        let new_descriptor = rename_data.new_descriptor().clone();
        let mut create_data = NewDescriptorData::new(label.clone());
        let description: Description = "Some new description\n".into();
        create_data.set_descriptor(old_descriptor.clone());
        create_data.set_description(description.clone());
        let create_message = GuiMessage::NewDescriptor(create_data.clone());
        gui.handle_message(create_message).unwrap();

        let rename_message = GuiMessage::RenameDescriptor(rename_data);
        gui.handle_message(rename_message).unwrap();

        assert_eq!(gui.get_selected_label(), Some(label));
        assert_eq!(gui.get_selected_descriptor(), Some(new_descriptor));
        assert_eq!(gui.get_description_text(), description.to_str());
    }

    #[test]
    fn delete_descriptor_deselects_it() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let new_descriptor_data = example_new_descriptor_data();
        let create_message = GuiMessage::NewDescriptor(new_descriptor_data.clone());
        gui.handle_message(create_message).unwrap();

        let label = new_descriptor_data.label().clone();
        let delete_message = GuiMessage::DeleteDescriptor(
            label.clone(),
            new_descriptor_data.descriptor().to_owned(),
        );
        gui.handle_message(delete_message).unwrap();

        assert_eq!(gui.get_selected_label(), Some(label));
        assert_eq!(gui.get_selected_descriptor(), None);
        assert_eq!(gui.get_description_text(), "\n");
    }

    #[test]
    fn new_history_item_sets_it_selected() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let data = example_new_history_data();
        let message = GuiMessage::NewHistoryItem(data.clone());
        gui.handle_message(message).unwrap();

        assert_eq!(gui.get_selected_year(), Some(data.year().to_owned()));
        assert_eq!(gui.get_selected_day(), Some(data.day().to_owned()));
        assert!(gui.get_selected_timestamp().is_some());
        assert_eq!(gui.get_history_text(), data.content().to_str().to_owned());
    }

    #[test]
    fn redate_history_item_sets_it_selected() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let new_history_data = example_new_history_data();
        let create_message = GuiMessage::NewHistoryItem(new_history_data.clone());
        gui.handle_message(create_message).unwrap();

        let timestamp = gui.get_selected_timestamp().unwrap();

        let old_year = new_history_data.year().clone();
        let old_day = new_history_data.day().clone();
        let mut redate_data = RedateHistoryData::new(timestamp, old_year, old_day);
        let new_year = old_year + 1;
        let new_day = old_day + 1;
        redate_data.set_new_year(new_year);
        redate_data.set_new_day(new_day);
        let redate_message = GuiMessage::RedateHistoryItem(redate_data.clone());
        gui.handle_message(redate_message).unwrap();

        assert_eq!(gui.get_selected_year(), Some(new_year));
        assert_eq!(gui.get_selected_day(), Some(new_day));
        assert_eq!(gui.get_selected_timestamp(), Some(timestamp));
        assert_eq!(
            gui.get_history_text(),
            new_history_data.content().to_str().to_owned()
        );
    }

    #[test]
    fn delete_history_item_deselects_all() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let new_history_data = example_new_history_data();
        let create_message = GuiMessage::NewHistoryItem(new_history_data.clone());
        gui.handle_message(create_message).unwrap();

        let timestamp = gui.get_selected_timestamp().unwrap();

        let delete_message = GuiMessage::DeleteHistoryItem(timestamp);
        gui.handle_message(delete_message).unwrap();

        assert_eq!(gui.get_selected_year(), None);
        assert_eq!(gui.get_selected_day(), None);
        assert_eq!(gui.get_selected_timestamp(), None);
        assert_eq!(gui.get_history_text(), "\n");
    }

    #[test]
    fn new_relationship_sets_it_selected() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let data = example_new_relationship_data();
        let message = GuiMessage::NewRelationship(data.clone());
        gui.handle_message(message).unwrap();

        assert_eq!(gui.get_selected_parent(), Some(data.parent().to_owned()));
        assert_eq!(gui.get_selected_child(), Some(data.child().to_owned()));
        assert_eq!(gui.get_selected_role(), Some(data.role().to_owned()));
    }

    #[test]
    fn creating_relationship_that_already_exists_produces_error() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let data = example_new_relationship_data();
        let message = GuiMessage::NewRelationship(data.clone());
        gui.handle_message(message).unwrap();
        let message = GuiMessage::NewRelationship(data.clone());
        let result = gui.handle_message(message);
        assert!(result.is_err());
    }

    #[test]
    fn changing_role_selects_relationship() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let create_data = example_new_relationship_data();
        let create_message = GuiMessage::NewRelationship(create_data.clone());
        gui.handle_message(create_message).unwrap();

        let parent = create_data.parent();
        let child = create_data.child();
        let role = create_data.role();
        let relationship = EntityRelationship {
            parent: parent.clone(),
            child: child.clone(),
            role: role.clone(),
        };

        let new_role = Role::from("New Role");
        let mut change_data = ChangeRoleData::new(relationship);
        change_data.set_new_role(new_role.clone());
        let change_message = GuiMessage::ChangeRole(change_data.clone());
        gui.handle_message(change_message).unwrap();

        assert_eq!(gui.get_selected_parent(), Some(parent.clone()));
        assert_eq!(gui.get_selected_child(), Some(child.clone()));
        assert_eq!(gui.get_selected_role(), Some(new_role));
    }

    #[test]
    fn deleting_relationships_deselects_all() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let create_data = example_new_relationship_data();
        let create_message = GuiMessage::NewRelationship(create_data.clone());
        gui.handle_message(create_message).unwrap();

        let parent = create_data.parent();
        let child = create_data.child();
        let role = create_data.role();
        let relationship = EntityRelationship {
            parent: parent.clone(),
            child: child.clone(),
            role: role.clone(),
        };

        let delete_message = GuiMessage::DeleteRelationship(relationship);
        gui.handle_message(delete_message).unwrap();

        assert_eq!(gui.get_selected_parent(), None);
        assert_eq!(gui.get_selected_child(), None);
        assert_eq!(gui.get_selected_role(), None);
    }
}
