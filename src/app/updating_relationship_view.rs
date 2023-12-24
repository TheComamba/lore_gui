use lorecore::sql::lore_database::LoreDatabase;

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
                state.update_parents(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, parent) => {
                state.parent_view_state.set_selected(parent);
                state.update_children(&self.lore_database)?;
                state.update_role(&self.lore_database)?;
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
                state.update_children(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, child) => {
                state.child_view_state.set_selected(child);
                state.update_parents(&self.lore_database)?;
                state.update_role(&self.lore_database)?;
            }
        };
        Ok(())
    }
}

impl RelationshipViewState {
    pub(super) fn reset_selections(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<(), LoreGuiError> {
        self.parent_view_state.set_selected_none();
        self.child_view_state.set_selected_none();
        self.current_role = None;
        self.update_parents(db)?;
        self.update_children(db)?;
        Ok(())
    }

    fn update_parents(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let parents = self.get_current_parents(db)?;
        self.parent_view_state.set_entries(parents);
        Ok(())
    }

    fn update_children(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let children = self.get_current_children(db)?;
        self.child_view_state.set_entries(children);
        Ok(())
    }

    fn update_role(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        self.current_role = self.get_current_role(db)?;
        Ok(())
    }
}
