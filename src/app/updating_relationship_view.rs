use super::SqlGui;
use crate::{
    db_col_view::ColViewMes, errors::LoreGuiError, relationship_view::RelationshipViewState,
};

impl SqlGui {
    pub(super) fn update_parent_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.relationship_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.parent_view_state.set_search_text(text);
                state.update_parents();
            }
            ColViewMes::Selected(_index, parent) => {
                state.parent_view_state.set_selected(parent);
                state.update_children();
                state.update_role();
            }
        };
        Ok(())
    }

    pub(super) fn update_child_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.relationship_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.child_view_state.set_search_text(text);
                state.update_children();
            }
            ColViewMes::Selected(_index, child) => {
                state.child_view_state.set_selected(child);
                state.update_parents();
                state.update_role();
            }
        };
        Ok(())
    }
}

impl RelationshipViewState {
    pub(super) fn reset_selections(&mut self) {
        self.parent_view_state.set_selected_none();
        self.child_view_state.set_selected_none();
        self.current_role = None;
        self.update_parents();
        self.update_children();
    }

    fn update_parents(&mut self) {
        let child = self.child_view_state.get_selected();
        let search_text = self.parent_view_state.get_search_text();
        self.parent_view_state
            .set_entries(self.get_parents(child.as_deref(), search_text));
    }

    fn update_children(&mut self) {
        let parent = self.parent_view_state.get_selected();
        let search_text = self.child_view_state.get_search_text();
        self.child_view_state
            .set_entries(self.get_children(parent.as_deref(), search_text));
    }

    fn update_role(&mut self) {
        let parent = match self.parent_view_state.get_selected() {
            Some(parent) => parent,
            None => {
                self.current_role = None;
                return;
            }
        };
        let child = match self.child_view_state.get_selected() {
            Some(child) => child,
            None => {
                self.current_role = None;
                return;
            }
        };
        self.current_role = self.get_role(parent, child);
    }
}
