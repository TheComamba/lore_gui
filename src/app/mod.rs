use super::{
    dialog::Dialog, entity_view::EntityViewState, history_view::HistoryViewState,
    relationship_view::RelationshipViewState,
};
use lorecore::sql::lore_database::LoreDatabase;

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

#[derive(Debug, Clone, Default)]
pub(crate) enum ViewType {
    #[default]
    Entity,
    History,
    Relationship,
}
