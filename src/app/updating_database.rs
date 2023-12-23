use super::SqlGui;
use crate::{errors::LoreGuiError, file_dialogs, user_preferences::store_database_path};
use lorecore::sql::{
    lore_database::LoreDatabase,
    search_text::{EntityColumnSearchParams, HistoryItemSearchParams, RelationshipSearchParams},
};
use std::path::PathBuf;

impl SqlGui {
    pub(super) fn update_database_derived_data(&mut self) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;

        let entity_columns = db
            .get_entity_columns(EntityColumnSearchParams::empty())
            .map_err(LoreGuiError::LoreCoreError)?;
        self.entity_view_state.set_entity_columns(entity_columns);
        self.entity_view_state
            .reset_selections(&self.lore_database)?;

        let history_items = db
            .get_history_items(HistoryItemSearchParams::empty())
            .map_err(LoreGuiError::LoreCoreError)?;
        self.history_view_state.set_history_items(history_items);
        self.history_view_state
            .reset_selections(&self.lore_database)?;

        let relationships = db
            .get_relationships(RelationshipSearchParams::empty())
            .map_err(LoreGuiError::LoreCoreError)?;
        self.relationship_view_state
            .set_relationships(relationships);
        self.relationship_view_state.reset_selections();
        Ok(())
    }

    pub(super) fn new_database_from_dialog(&mut self) -> Result<(), LoreGuiError> {
        let path = match file_dialogs::new() {
            Some(path) => path,
            None => return Ok(()),
        };
        self.new_database(path.clone())?;
        store_database_path(path)?;
        Ok(())
    }

    pub(super) fn new_database(&mut self, path: PathBuf) -> Result<(), LoreGuiError> {
        self.lore_database = Some(LoreDatabase::open(path).map_err(LoreGuiError::LoreCoreError)?);
        self.update_database_derived_data()?;
        Ok(())
    }

    pub(super) fn open_database_from_dialog(&mut self) -> Result<(), LoreGuiError> {
        let path = match file_dialogs::open() {
            Some(path) => path,
            None => return Ok(()),
        };
        self.open_database(path.clone())?;
        store_database_path(path)?;
        Ok(())
    }

    pub(super) fn open_database(&mut self, path: PathBuf) -> Result<(), LoreGuiError> {
        self.lore_database = Some(LoreDatabase::open(path).map_err(LoreGuiError::LoreCoreError)?);
        self.update_database_derived_data()?;
        Ok(())
    }
}
