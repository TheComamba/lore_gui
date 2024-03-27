use super::{SqlGui, ViewType};
use crate::{
    dialog::{
        new_descriptor::NewDescriptorData, new_entity::NewEntityData,
        new_history_item::NewHistoryData, new_relationship::NewRelationshipData,
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
    DeleteEntity(String),
    NewDescriptor(NewDescriptorData),
    DeleteDescriptor(String, String),
    NewHistoryItem(NewHistoryData),
    DeleteHistoryItem(i64),
    NewRelationship(NewRelationshipData),
    DeleteRelationship(String, String, String),
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
            GuiMes::DeleteEntity(label) => todo!(),
            GuiMes::NewDescriptor(data) => self.write_new_descriptor(data)?,
            GuiMes::DeleteDescriptor(label, descriptor) => todo!(),
            GuiMes::NewHistoryItem(data) => self.write_new_history(data)?,
            GuiMes::DeleteHistoryItem(timestamp) => todo!(),
            GuiMes::NewRelationship(data) => self.write_new_relationship(data)?,
            GuiMes::DeleteRelationship(parent, child, role) => todo!(),
        }
        Ok(())
    }
}
