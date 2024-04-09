use lorecore::sql::relationships::EntityRelationship;

use super::{SqlGui, ViewType};
use crate::{
    dialog::{
        change_role::ChangeRoleData, new_descriptor::NewDescriptorData, new_entity::NewEntityData,
        new_history_item::NewHistoryData, new_relationship::NewRelationshipData,
        redate_history::RedateHistoryData, relabel_entity::RelabelEntityData,
        rename_descriptor::RenameDescriptorData,
    },
    entity_view::EntityViewMessage,
    errors::LoreGuiError,
    history_view::HistoryViewMessage,
    relationship_view::RelationshipViewMessage,
};

#[derive(Debug, Clone)]
pub(crate) enum GuiMes {
    ViewSelected(ViewType),
    NewDatabase,
    OpenDatabase,
    EntityViewUpd(EntityViewMessage),
    HistoryViewUpd(HistoryViewMessage),
    RelationshipViewUpd(RelationshipViewMessage),
    DialogClosed,
    NewEntity(NewEntityData),
    RelabelEntity(RelabelEntityData),
    DeleteEntity(String),
    NewDescriptor(NewDescriptorData),
    RenameDescriptor(RenameDescriptorData),
    DeleteDescriptor(String, String),
    NewHistoryItem(NewHistoryData),
    RedateHistoryItem(RedateHistoryData),
    DeleteHistoryItem(i64),
    NewRelationship(NewRelationshipData),
    ChangeRole(ChangeRoleData),
    DeleteRelationship(EntityRelationship),
}

impl SqlGui {
    pub(super) fn handle_message(&mut self, message: GuiMes) -> Result<(), LoreGuiError> {
        match message {
            GuiMes::ViewSelected(view) => self.selected_view = view,
            GuiMes::NewDatabase => self.new_database_from_dialog()?,
            GuiMes::OpenDatabase => self.open_database_from_dialog()?,
            GuiMes::EntityViewUpd(event) => self.update_entity_view(event)?,
            GuiMes::HistoryViewUpd(event) => self.update_history_view(event)?,
            GuiMes::RelationshipViewUpd(event) => self.update_relationship_view(event)?,
            GuiMes::DialogClosed => self.dialog = None,
            GuiMes::NewEntity(data) => self.write_new_entity(data)?,
            GuiMes::RelabelEntity(data) => self.relable_entity(data)?,
            GuiMes::DeleteEntity(label) => self.delete_entity(label)?,
            GuiMes::NewDescriptor(data) => self.write_new_descriptor(data)?,
            GuiMes::RenameDescriptor(data) => self.change_descriptor(data)?,
            GuiMes::DeleteDescriptor(label, descriptor) => {
                self.delete_descriptor(label, descriptor)?
            }
            GuiMes::NewHistoryItem(data) => self.write_new_history(data)?,
            GuiMes::RedateHistoryItem(data) => self.redate_history_item(data)?,
            GuiMes::DeleteHistoryItem(timestamp) => self.delete_history_item(timestamp)?,
            GuiMes::NewRelationship(data) => self.write_new_relationship(data)?,
            GuiMes::ChangeRole(data) => self.change_relationship_role(data)?,
            GuiMes::DeleteRelationship(relationship) => self.delete_relationship(relationship)?,
        }
        Ok(())
    }
}
