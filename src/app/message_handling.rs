use lorecore::types::{
    descriptor::Descriptor, label::Label, relationship::EntityRelationship, timestamp::Timestamp,
};

use super::{SqlGui, ViewType};
use crate::{
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

impl SqlGui {
    pub(super) fn handle_message(&mut self, message: GuiMessage) -> Result<(), LoreGuiError> {
        match message {
            GuiMessage::ViewSelected(view) => self.selected_view = view,
            GuiMessage::NewDatabase => self.new_database_from_dialog()?,
            GuiMessage::OpenDatabase => self.open_database_from_dialog()?,
            GuiMessage::EntityViewUpd(event) => self.update_entity_view(event)?,
            GuiMessage::HistoryViewUpd(event) => self.update_history_view(event)?,
            GuiMessage::RelationshipViewUpd(event) => self.update_relationship_view(event)?,
            GuiMessage::DialogUpdate(event) => self.dialog.as_mut().unwrap().update(event),
            GuiMessage::DialogSubmit => {
                self.handle_message(self.dialog.as_ref().unwrap().submit())?
            }
            GuiMessage::DialogClosed => self.dialog = None,
            GuiMessage::NewEntity(data) => self.write_new_entity(data)?,
            GuiMessage::RelabelEntity(data) => self.relable_entity(data)?,
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
        Ok(())
    }
}
