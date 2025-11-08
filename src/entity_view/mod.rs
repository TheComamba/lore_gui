use iced::widget::text_editor;
use lorecore::{
    extractions::{extract_descriptors, extract_labels},
    sql::{
        lore_database::LoreDatabase,
        search_params::{EntityColumnSearchParams, SqlSearchText},
    },
    types::*,
};

use crate::{
    db_col_view::{entry::DbColViewEntry, ColViewMes},
    dialog::{relabel_entity::RelabelEntityData, rename_descriptor::RenameDescriptorData},
    editor::EditorState,
    errors::LoreGuiError,
};

use super::db_col_view::state::DbColViewState;

pub(crate) mod widget;

pub(super) struct EntityViewState {
    pub(super) display_protected: bool,
    pub(super) label_view_state: DbColViewState<Label>,
    pub(super) descriptor_view_state: DbColViewState<Descriptor>,
    pub(super) current_description: EditorState,
}

#[derive(Debug, Clone)]
pub(super) enum EntityViewMessage {
    NewEntity,
    RelabelEntity(RelabelEntityData),
    DeleteEntity(Label),
    NewDescriptor(Label),
    RenameDescriptor(RenameDescriptorData),
    DeleteDescriptor(Label, Descriptor),
    LabelViewUpdate(ColViewMes<Label>),
    DescriptorViewUpdate(ColViewMes<Descriptor>),
    DescriptionUpdate(text_editor::Action),
    DescriptionDiscard,
    DescriptionSave,
}

impl EntityViewState {
    pub(super) fn new(display_protected: bool) -> Self {
        Self {
            display_protected,
            label_view_state: DbColViewState::default(),
            descriptor_view_state: DbColViewState::default(),
            current_description: EditorState::default(),
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
        let mut labels = extract_labels(&entity_columns);
        if !self.display_protected {
            labels.retain(|label| !label.is_protected());
        }
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
        let label = match self.get_selected_label() {
            Some(label) => Some(SqlSearchText::exact(label.to_str())),
            None => return Ok(vec![]),
        };

        let descriptor_search_text = self
            .descriptor_view_state
            .get_search_text()
            .map(SqlSearchText::partial);
        let search_params = EntityColumnSearchParams::new(label, descriptor_search_text);
        let entity_columns = db.read_entity_columns(search_params)?;
        let mut descriptors = extract_descriptors(&entity_columns);
        if !self.display_protected {
            descriptors.retain(|descriptor| !descriptor.is_protected());
        }
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
        let label = match self.get_selected_label() {
            Some(label) => Some(SqlSearchText::exact(label.to_str())),
            None => return Ok(Description::NONE),
        };
        let descriptor = match self.get_selected_descriptor() {
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

    pub(super) fn get_selected_label(&self) -> Option<Label> {
        self.label_view_state.get_selected().0.clone()
    }

    pub(super) fn set_selected_label(&mut self, label: Option<Label>) {
        self.label_view_state.set_selected(DbColViewEntry(label));
    }

    pub(super) fn get_selected_descriptor(&self) -> Option<Descriptor> {
        self.descriptor_view_state.get_selected().0.clone()
    }

    pub(super) fn set_selected_descriptor(&mut self, descriptor: Option<Descriptor>) {
        self.descriptor_view_state
            .set_selected(DbColViewEntry(descriptor));
    }

    pub(super) fn get_description_text(&self) -> String {
        self.current_description.get_text()
    }

    pub(super) fn set_description_text(&mut self, text: &str) {
        self.current_description = EditorState::new(text);
    }

    pub(super) fn set_display_protected(&mut self, display_protected: bool) {
        self.display_protected = display_protected;
    }
}

impl Default for EntityViewState {
    fn default() -> Self {
        Self::new(false)
    }
}
