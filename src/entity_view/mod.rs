use iced::widget::text_editor;
use lorecore::{
    extractions::{extract_descriptors, extract_labels},
    sql::{
        lore_database::LoreDatabase,
        search_params::{EntityColumnSearchParams, SqlSearchText},
    },
    types::{description::Description, descriptor::Descriptor, label::Label},
};

use crate::{
    db_col_view::ColViewMes,
    dialog::{relabel_entity::RelabelEntityData, rename_descriptor::RenameDescriptorData},
    errors::LoreGuiError,
};

use super::db_col_view::state::DbColViewState;

mod widget;

pub(super) struct EntityView<'a> {
    state: &'a EntityViewState,
}

pub(super) struct EntityViewState {
    pub(super) label_view_state: DbColViewState<Label>,
    pub(super) descriptor_view_state: DbColViewState<Descriptor>,
    pub(super) current_description: text_editor::Content,
}

#[derive(Debug, Clone)]
pub(super) enum EntityViewMessage {
    NewEntity,
    RelabelEntity(RelabelEntityData),
    DeleteEntity(Label),
    NewDescriptor(Label),
    RenameDescriptor(RenameDescriptorData),
    DeleteDescriptor(Label, Descriptor),
    LabelViewUpd(ColViewMes<Label>),
    DescriptorViewUpd(ColViewMes<Descriptor>),
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
    ) -> Result<Vec<Label>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };

        let label_search_text = self
            .label_view_state
            .get_search_text()
            .map(SqlSearchText::partial);
        let search_params = EntityColumnSearchParams::new(label_search_text, None);
        let entity_columns = db.read_entity_columns(search_params)?;
        let labels = extract_labels(&entity_columns);
        Ok(labels)
    }

    pub(super) fn get_current_descriptors(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Descriptor>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let label = match &self.label_view_state.get_selected().0 {
            Some(label) => Some(SqlSearchText::exact(label.to_str())),
            None => return Ok(vec![]),
        };

        let descriptor_search_text = self
            .descriptor_view_state
            .get_search_text()
            .map(SqlSearchText::partial);
        let search_params = EntityColumnSearchParams::new(label, descriptor_search_text);
        let entity_columns = db.read_entity_columns(search_params)?;
        let descriptors = extract_descriptors(&entity_columns);
        Ok(descriptors)
    }

    pub(super) fn get_current_description(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Description, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(Description::NONE),
        };
        let label = match &self.label_view_state.get_selected().0 {
            Some(label) => Some(SqlSearchText::exact(label.to_str())),
            None => return Ok(Description::NONE),
        };
        let descriptor = match &self.descriptor_view_state.get_selected().0 {
            Some(descriptor) => Some(SqlSearchText::exact(descriptor.to_str())),
            None => return Ok(Description::NONE),
        };

        let search_params = EntityColumnSearchParams::new(label, descriptor);
        let entity_columns = db.read_entity_columns(search_params)?;

        if entity_columns.len() > 1 {
            return Err(LoreGuiError::MultipleResults);
        }

        let description = entity_columns
            .first()
            .map(|col| col.description.clone())
            .unwrap_or(Description::NONE);

        Ok(description)
    }
}

impl Default for EntityViewState {
    fn default() -> Self {
        Self::new()
    }
}
