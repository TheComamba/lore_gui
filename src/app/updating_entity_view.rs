use super::SqlGui;
use crate::{
    db_col_view::{state::DbColViewState, ColViewMes},
    dialog::new_entity::{NewEntityData, NewEntityDialog},
    entity_view::EntityViewState,
    errors::LoreGuiError,
};
use lorecore::sql::{entity::EntityColumn, lore_database::LoreDatabase};

impl SqlGui {
    pub(super) fn update_label_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let db = self.get_database()?;
        let state = &mut self.entity_view_state;
        match event {
            ColViewMes::New => self.dialog = Some(Box::new(NewEntityDialog::new())),
            ColViewMes::SearchFieldUpd(text) => {
                state.label_view_state.set_search_text(text);
                state.update_labels(db)?;
            }
            ColViewMes::Selected(_index, label) => {
                state.label_view_state.set_selected(label);
                state.descriptor_view_state.set_selected_none();
                state.update_descriptors(db)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_descriptor_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let db = self.get_database()?;
        let state = &mut self.entity_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.descriptor_view_state.set_search_text(text);
                state.update_descriptors(db)?;
            }
            ColViewMes::Selected(_index, descriptor) => {
                state.descriptor_view_state.set_selected(descriptor);
                state.update_description(db)?;
            }
        };
        Ok(())
    }

    pub(super) fn write_new_entity(&mut self, data: NewEntityData) -> Result<(), LoreGuiError> {
        let db = self.get_database()?;
        let label = data.get_label().to_string();
        data.write_to_database(db)?;
        self.update_label_view(ColViewMes::SearchFieldUpd(label))?;
        self.dialog = None;
        Ok(())
    }
}

impl EntityViewState {
    pub(super) fn reset(&mut self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        self.reset_selections();
        self.update_labels(db)?;
        Ok(())
    }

    fn reset_selections(&mut self) {
        self.label_view_state.set_selected_none();
        self.descriptor_view_state.set_selected_none();
        self.current_description = None;
    }

    fn update_labels(&mut self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        self.label_view_state.set_entries(
            db.get_entity_labels(self.label_view_state.get_sql_search_text())
                .map_err(LoreGuiError::LoreCoreError)?,
        );
        self.update_descriptors(db)?;
        Ok(())
    }

    fn update_descriptors(&mut self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        let label = self.label_view_state.get_selected();
        match label {
            Some(label) => {
                let search_text = self.descriptor_view_state.get_sql_search_text();
                self.descriptor_view_state.set_entries(
                    db.get_descriptors(&label, search_text)
                        .map_err(LoreGuiError::LoreCoreError)?,
                );
            }
            None => {
                self.descriptor_view_state = DbColViewState::default();
            }
        }

        self.update_description(db)?;
        Ok(())
    }

    fn update_description(&mut self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        let label = match self.label_view_state.get_selected() {
            Some(label) => label,
            None => {
                self.current_description = None;
                return Ok(());
            }
        };
        let descriptor = match self.descriptor_view_state.get_selected() {
            Some(descriptor) => descriptor,
            None => {
                self.current_description = None;
                return Ok(());
            }
        };

        self.current_description = db
            .get_description(label, descriptor)
            .map_err(LoreGuiError::LoreCoreError)?;

        Ok(())
    }

    fn new_descriptor(&mut self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        let label = match self.label_view_state.get_selected().as_ref() {
            Some(label) => label.clone(),
            None => {
                return Err(LoreGuiError::InputError(
                    "No label selected for which to create new descriptor.".to_string(),
                ));
            }
        };
        let descriptor = self.descriptor_view_state.get_search_text().to_string();
        if descriptor.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot create empty descriptor.".to_string(),
            ));
        }
        let description = None;
        let new_col = EntityColumn {
            label,
            descriptor,
            description,
        };
        db.write_entity_columns(vec![new_col])
            .map_err(LoreGuiError::LoreCoreError)?;
        self.update_descriptors(db)?;
        Ok(())
    }
}
