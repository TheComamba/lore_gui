use super::SqlGui;
use crate::{
    db_col_view::ColViewMes,
    dialog::{
        new_descriptor::{NewDescriptorData, NewDescriptorDialog},
        new_entity::{NewEntityData, NewEntityDialog},
    },
    entity_view::EntityViewState,
    errors::LoreGuiError,
};
use lorecore::sql::{
    entity::{get_descriptors, get_labels},
    lore_database::LoreDatabase,
    search_text::EntityColumnSearchParams,
};

impl SqlGui {
    pub(super) fn update_label_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.entity_view_state;
        match event {
            ColViewMes::New => self.dialog = Some(Box::new(NewEntityDialog::new())),
            ColViewMes::SearchFieldUpd(text) => {
                state.label_view_state.set_search_text(text);
                state.update_labels(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, label) => {
                state.label_view_state.set_selected(label);
                state.descriptor_view_state.set_selected_none();
                state.update_descriptors(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_descriptor_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.entity_view_state;
        match event {
            ColViewMes::New => {
                let label = state.label_view_state.get_selected().as_ref().ok_or(
                    LoreGuiError::InputError(
                        "No label selected for which to create new descriptor.".to_string(),
                    ),
                )?;
                self.dialog = Some(Box::new(NewDescriptorDialog::new(label.clone())));
            }
            ColViewMes::SearchFieldUpd(text) => {
                state.descriptor_view_state.set_search_text(text);
                state.update_descriptors(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, descriptor) => {
                state.descriptor_view_state.set_selected(descriptor);
                state.update_description();
            }
        };
        Ok(())
    }

    pub(super) fn write_new_entity(&mut self, data: NewEntityData) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let label = data.get_label().to_string();
        data.write_to_database(db)?;
        self.update_label_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_label_view(ColViewMes::Selected(0, label))?;
        self.dialog = None;
        Ok(())
    }

    pub(super) fn write_new_descriptor(
        &mut self,
        data: NewDescriptorData,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let descriptor = data.get_descriptor().to_string();
        data.write_to_database(db)?;
        self.update_descriptor_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_descriptor_view(ColViewMes::Selected(0, descriptor))?;
        self.dialog = None;
        Ok(())
    }
}

impl EntityViewState {
    pub(super) fn reset_selections(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<(), LoreGuiError> {
        self.label_view_state.set_selected_none();
        self.descriptor_view_state.set_selected_none();
        self.current_description = None;
        self.update_labels(db)?;
        Ok(())
    }

    fn update_labels(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let labels = if let Some(db) = db {
            let search_text = self.label_view_state.get_search_text().map(|t| (t, false));
            let search_params = EntityColumnSearchParams::new(search_text, None);
            let entity_columns = db
                .get_entity_columns(search_params)
                .map_err(LoreGuiError::LoreCoreError)?;
            get_labels(&entity_columns)
        } else {
            vec![]
        };
        self.label_view_state.set_entries(labels);
        self.update_descriptors(db)?;
        Ok(())
    }

    fn update_descriptors(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let descriptors = self.get_descriptors(db)?;
        self.descriptor_view_state.set_entries(descriptors);
        self.update_description();
        Ok(())
    }

    fn get_descriptors(&mut self, db: &Option<LoreDatabase>) -> Result<Vec<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let label = match self.label_view_state.get_selected() {
            Some(label) => Some((label.as_str(), true)),
            None => return Ok(vec![]),
        };

        let search_text = self
            .descriptor_view_state
            .get_search_text()
            .map(|t| (t, false));
        let search_params = EntityColumnSearchParams::new(label, search_text);
        let entity_columns = db
            .get_entity_columns(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let descriptors = get_descriptors(&entity_columns);
        Ok(descriptors)
    }

    fn update_description(&mut self) {
        let label = match self.label_view_state.get_selected() {
            Some(label) => label,
            None => {
                self.current_description = None;
                return;
            }
        };
        let descriptor = match self.descriptor_view_state.get_selected() {
            Some(descriptor) => descriptor,
            None => {
                self.current_description = None;
                return;
            }
        };

        self.current_description = self.get_description(label, descriptor);
    }
}
