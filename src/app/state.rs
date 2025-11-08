use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{Child, Day, Descriptor, Label, Parent, Role, Timestamp, Year},
};

use crate::{
    dialog::{error::ErrorDialog, Dialog},
    entity_view::EntityViewState,
    history_view::HistoryViewState,
    relationship_view::RelationshipViewState,
    user_preferences::load_database_path,
};

pub(crate) struct GuiState {
    pub(super) selected_view: ViewType,
    pub(super) display_protected: bool,
    pub(super) entity_view_state: EntityViewState,
    pub(super) history_view_state: HistoryViewState,
    pub(super) relationship_view_state: RelationshipViewState,
    pub(super) lore_database: Option<LoreDatabase>,
    pub(crate) dialog: Option<Box<dyn Dialog>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) enum ViewType {
    #[default]
    Entity,
    History,
    Relationship,
}

impl Default for GuiState {
    fn default() -> Self {
        let mut gui = GuiState {
            selected_view: ViewType::default(),
            display_protected: false,
            entity_view_state: EntityViewState::default(),
            history_view_state: HistoryViewState::default(),
            relationship_view_state: RelationshipViewState::default(),
            lore_database: None,
            dialog: None,
        };
        if let Some(path) = load_database_path() {
            match gui.initialise(path) {
                Ok(_) => (),
                Err(e) => gui.dialog = Some(Box::new(ErrorDialog::new(e))),
            };
        }
        gui
    }
}

impl GuiState {
    pub(crate) fn get_selected_label(&self) -> Option<Label> {
        self.entity_view_state.get_selected_label()
    }

    pub(crate) fn set_selected_label(&mut self, label: Option<Label>) {
        self.entity_view_state.set_selected_label(label);
    }

    pub(crate) fn get_selected_descriptor(&self) -> Option<Descriptor> {
        self.entity_view_state.get_selected_descriptor()
    }

    pub(crate) fn set_selected_descriptor(&mut self, descriptor: Option<Descriptor>) {
        self.entity_view_state.set_selected_descriptor(descriptor);
    }

    pub(crate) fn get_description_text(&self) -> String {
        self.entity_view_state.get_description_text()
    }

    #[cfg(test)]
    pub(crate) fn set_description_text(&mut self, text: &str) {
        self.entity_view_state.set_description_text(text);
    }

    #[cfg(test)]
    pub(crate) fn get_selected_year(&self) -> Option<Year> {
        self.history_view_state.get_selected_year()
    }

    pub(crate) fn set_selected_year(&mut self, year: Option<Year>) {
        self.history_view_state.set_selected_year(year);
    }

    #[cfg(test)]
    pub(crate) fn get_selected_day(&self) -> Option<Day> {
        self.history_view_state.get_selected_day()
    }

    pub(crate) fn set_selected_day(&mut self, day: Option<Day>) {
        self.history_view_state.set_selected_day(day);
    }

    pub(crate) fn get_selected_timestamp(&self) -> Option<Timestamp> {
        self.history_view_state.get_selected_timestamp()
    }

    pub(crate) fn set_selected_timestamp(&mut self, timestamp: Option<Timestamp>) {
        self.history_view_state.set_selected_timestamp(timestamp);
    }

    pub(crate) fn get_history_text(&self) -> String {
        self.history_view_state.get_content_text()
    }

    pub(crate) fn set_history_text(&mut self, text: &str) {
        self.history_view_state.set_content_text(text);
    }

    #[cfg(test)]
    pub(crate) fn get_selected_parent(&self) -> Option<Parent> {
        self.relationship_view_state.get_selected_parent()
    }

    pub(crate) fn set_selected_parent(&mut self, parent: Option<Parent>) {
        self.relationship_view_state.set_selected_parent(parent);
    }

    #[cfg(test)]
    pub(crate) fn get_selected_child(&self) -> Option<Child> {
        self.relationship_view_state.get_selected_child()
    }

    pub(crate) fn set_selected_child(&mut self, child: Option<Child>) {
        self.relationship_view_state.set_selected_child(child);
    }

    #[cfg(test)]
    pub(crate) fn get_selected_role(&self) -> Option<Role> {
        self.relationship_view_state.get_selected_role()
    }

    pub(crate) fn set_selected_role(&mut self, role: Option<Role>) {
        self.relationship_view_state.set_selected_role(role);
    }
}
