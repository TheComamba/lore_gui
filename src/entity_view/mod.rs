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
    pub(super) fn new(entity_columns: Vec<EntityColumn>) -> Self {
        Self {
            entity_columns,
            label_view_state: DbColViewState::default(),
            descriptor_view_state: DbColViewState::default(),
            current_description: None,
        }
    }

    pub(super) fn set_entity_columns(&mut self, entity_columns: Vec<EntityColumn>) {
        self.entity_columns = entity_columns;
    }

    pub(super) fn get_descriptors(&self, label: &str, search_text: Option<&str>) -> Vec<String> {
        let mut descriptors: Vec<String> = self
            .entity_columns
            .iter()
            .filter(|e| e.label == label)
            .filter(|e| match search_text {
                Some(ref search_text) => e.descriptor.contains(search_text),
                None => true,
            })
            .map(|col| col.descriptor.clone())
            .collect();
        descriptors.sort();
        descriptors.dedup();
        descriptors
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
