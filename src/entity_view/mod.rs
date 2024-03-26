use super::db_col_view::state::DbColViewState;
use crate::{db_col_view::ColViewMes, errors::LoreGuiError};
use iced::widget::text_editor;
use lorecore::sql::{
    entity::{extract_descriptors, extract_labels},
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
    pub(super) current_description: text_editor::Content,
}

#[derive(Debug, Clone)]
pub(super) enum EntityViewMessage {
    NewEntity,
    NewDescriptor(String),
    LabelViewUpd(ColViewMes),
    DescriptorViewUpd(ColViewMes),
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
            current_description: text_editor::Content::with_text(""),
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
        let entity_columns = db.read_entity_columns(search_params)?;
        let labels = extract_labels(&entity_columns);
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
        let entity_columns = db.read_entity_columns(search_params)?;
        let descriptors = extract_descriptors(&entity_columns);
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
        let entity_columns = db.read_entity_columns(search_params)?;

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
