use super::{SqlGui, ViewType};
use crate::{
    dialog::{
        new_descriptor::NewDescriptorData, new_entity::NewEntityData,
        new_history_item::NewHistoryData,
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
    NewDescriptor(NewDescriptorData),
    NewHistoryItem(NewHistoryData),
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
            GuiMes::NewDescriptor(data) => self.write_new_descriptor(data)?,
            GuiMes::NewHistoryItem(data) => self.write_new_history(data)?,
        }
        Ok(())
    }
}
