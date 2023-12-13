use lorecore::sql::entity::EntityColumn;

use super::db_col_view::state::DbColViewState;

mod widget;

pub(super) struct EntityView<'a> {
    state: &'a EntityViewState,
}

pub(super) struct EntityViewState {
    entity_columns: Vec<EntityColumn>,
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
    pub(super) fn new(labels: Vec<String>) -> Self {
        Self {
            entity_columns: vec![],
            label_view_state: DbColViewState::new(labels),
            descriptor_view_state: DbColViewState::default(),
            current_description: None,
        }
    }

    pub(super) fn get_labels(&self, search_text: Option<&str>) -> Vec<String> {
        self.entity_columns
            .iter()
            .filter(|e| match search_text {
                Some(ref search_text) => e.label.contains(search_text),
                None => true,
            })
            .map(|col| col.label.clone())
            .collect()
    }

    pub(super) fn get_descriptors(&self, label: &str, search_text: Option<&str>) -> Vec<String> {
        self.entity_columns
            .iter()
            .filter(|e| e.label == label)
            .filter(|e| match search_text {
                Some(ref search_text) => e.descriptor.contains(search_text),
                None => true,
            })
            .map(|col| col.descriptor.clone())
            .collect()
    }

    pub(super) fn get_description(&self, label: &str, descriptor: &str) -> Option<String> {
        self.entity_columns
            .iter()
            .find(|e| e.label == label && e.descriptor == descriptor)
            .map(|col| col.description.clone())?
    }
}

impl Default for EntityViewState {
    fn default() -> Self {
        Self::new(vec![])
    }
}
