use lorecore::{sql::lore_database::LoreDatabase, types::*};

use super::{
    dialog::Dialog, entity_view::EntityViewState, history_view::HistoryViewState,
    relationship_view::RelationshipViewState,
};

pub(super) mod message_handling;
mod updating_database;
mod updating_entity_view;
mod updating_history_view;
mod updating_relationship_view;
mod widget;

pub(crate) struct SqlGui {
    selected_view: ViewType,
    entity_view_state: EntityViewState,
    history_view_state: HistoryViewState,
    relationship_view_state: RelationshipViewState,
    lore_database: Option<LoreDatabase>,
    pub(crate) dialog: Option<Box<dyn Dialog>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) enum ViewType {
    #[default]
    Entity,
    History,
    Relationship,
}

impl SqlGui {
    pub(crate) fn selected_label(&self) -> Option<Label> {
        self.entity_view_state.selected_label()
    }

    pub(crate) fn selected_descriptor(&self) -> Option<Descriptor> {
        self.entity_view_state.selected_descriptor()
    }

    #[cfg(test)]
    pub(crate) fn description_text(&self) -> String {
        self.entity_view_state.description_text()
    }

    #[cfg(test)]
    pub(crate) fn selected_year(&self) -> Option<Year> {
        self.history_view_state.selected_year()
    }

    #[cfg(test)]
    pub(crate) fn selected_day(&self) -> Option<Day> {
        self.history_view_state.selected_day()
    }

    pub(crate) fn selected_timestamp(&self) -> Option<Timestamp> {
        self.history_view_state.selected_timestamp()
    }

    pub(crate) fn history_text(&self) -> String {
        self.history_view_state.content_text()
    }

    #[cfg(test)]
    pub(crate) fn selected_parent(&self) -> Option<Parent> {
        self.relationship_view_state.selected_parent()
    }

    #[cfg(test)]
    pub(crate) fn selected_child(&self) -> Option<Child> {
        self.relationship_view_state.selected_child()
    }

    #[cfg(test)]
    pub(crate) fn selected_role(&self) -> Option<Role> {
        self.relationship_view_state.selected_role()
    }
}
