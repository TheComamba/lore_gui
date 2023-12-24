use super::db_col_view::state::DbColViewState;
use crate::errors::LoreGuiError;
use lorecore::sql::{
    entity::{get_descriptors, get_labels},
    lore_database::LoreDatabase,
    search_params::{EntityColumnSearchParams, SqlSearchText},
};

mod widget;

pub(super) struct EntityView<'a> {
    state: &'a EntityViewState,
}

pub(super) struct EntityViewState {
    pub(super) label_view_state: DbColViewState,
    pub(super) descriptor_view_state: DbColViewState,
    pub(super) current_description: Option<String>,
}

impl<'a> EntityView<'a> {
    pub(super) fn new(state: &'a EntityViewState) -> Self {
        Self { state }
    }
}

impl EntityViewState {
    pub(super) fn new() -> Self {
        Self {
            label_view_state: DbColViewState::default(),
            descriptor_view_state: DbColViewState::default(),
            current_description: None,
        }
    }

    pub(super) fn get_current_labels(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };

        let label_search_text = self
            .label_view_state
            .get_search_text()
            .map(|t| SqlSearchText::partial(t));
        let search_params = EntityColumnSearchParams::new(label_search_text, None);
        let entity_columns = db
            .get_entity_columns(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let labels = get_labels(&entity_columns);
        Ok(labels)
    }

    pub(super) fn get_current_descriptors(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let label = match self.label_view_state.get_selected() {
            Some(label) => Some(SqlSearchText::exact(label.as_str())),
            None => return Ok(vec![]),
        };

        let descriptor_search_text = self
            .descriptor_view_state
            .get_search_text()
            .map(|t| SqlSearchText::partial(t));
        let search_params = EntityColumnSearchParams::new(label, descriptor_search_text);
        let entity_columns = db
            .get_entity_columns(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let descriptors = get_descriptors(&entity_columns);
        Ok(descriptors)
    }

    pub(super) fn get_current_description(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Option<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(None),
        };
        let label = match self.label_view_state.get_selected() {
            Some(label) => Some(SqlSearchText::exact(label.as_str())),
            None => return Ok(None),
        };
        let descriptor = match self.descriptor_view_state.get_selected() {
            Some(descriptor) => Some(SqlSearchText::exact(descriptor.as_str())),
            None => return Ok(None),
        };

        let search_params = EntityColumnSearchParams::new(label, descriptor);
        let entity_columns = db
            .get_entity_columns(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;

        if entity_columns.len() > 1 {
            return Err(LoreGuiError::MultipleResults);
        }

        let description = entity_columns
            .first()
            .and_then(|col| col.description.clone());

        Ok(description)
    }
}

impl Default for EntityViewState {
    fn default() -> Self {
        Self::new()
    }
}
