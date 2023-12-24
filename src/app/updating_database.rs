use super::SqlGui;
use crate::{errors::LoreGuiError, file_dialogs, user_preferences::store_database_path};
use lorecore::sql::lore_database::LoreDatabase;
use std::path::PathBuf;

impl SqlGui {
    pub(super) fn update_database_derived_data(&mut self) -> Result<(), LoreGuiError> {
        self.entity_view_state
            .reset_selections(&self.lore_database)?;
        self.history_view_state
            .reset_selections(&self.lore_database)?;
        self.relationship_view_state
            .reset_selections(&self.lore_database)?;
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
