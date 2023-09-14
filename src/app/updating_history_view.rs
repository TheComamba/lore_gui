use crate::{
    db_col_view::{state::DbColViewState, ColViewMes},
    errors::LoreGuiError,
    history_view::HistoryViewState,
};
use lorecore::sql::lore_database::LoreDatabase;

use super::SqlGui;

impl SqlGui {
    pub(super) fn update_year_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.year_view_state.set_search_text(text);
                state.update_years(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, year) => {
                state.year_view_state.set_selected(year);
                state.day_view_state.set_selected_none();
                state.update_days(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_day_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.day_view_state.set_search_text(text);
                state.update_days(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, day) => {
                state.day_view_state.set_selected(day);
                state.label_view_state.set_selected_none();
                state.update_labels(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_history_label_view(
        &mut self,
        event: ColViewMes,
    ) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.label_view_state.set_search_text(text);
                state.update_labels(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, label) => {
                state.label_view_state.set_selected(label);
                state.update_content(&self.lore_database)?;
            }
        };
        Ok(())
    }
}

impl HistoryViewState {
    pub(super) fn reset(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        self.reset_selections();
        self.update_years(db)?;
        Ok(())
    }

    fn reset_selections(&mut self) {
        self.year_view_state.set_selected_none();
        self.day_view_state.set_selected_none();
        self.label_view_state.set_selected_none();
        self.current_content = String::new();
    }

    fn update_years(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        match db {
            Some(db) => {
                let years = db
                    .get_all_years()
                    .map_err(LoreGuiError::LoreCoreError)?
                    .iter()
                    .map(|y| y.to_string())
                    .collect();
                self.year_view_state.set_entries(years);
            }
            None => self.year_view_state = DbColViewState::default(),
        }
        self.update_days(db)?;
        Ok(())
    }

    fn optional_int_to_string(opt: &Option<i32>) -> String {
        match opt {
            None => String::new(),
            Some(i) => i.to_string(),
        }
    }

    fn update_days(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let year = self.year_view_state.get_selected_int()?;
        if let Some(db) = db {
            if let Some(year) = year {
                let days = db
                    .get_days(year)
                    .map_err(LoreGuiError::LoreCoreError)?
                    .iter()
                    .map(Self::optional_int_to_string)
                    .collect();
                self.day_view_state.set_entries(days);
            } else {
                self.day_view_state = DbColViewState::default();
            }
        } else {
            self.day_view_state = DbColViewState::default();
        }
        self.update_labels(db)?;
        Ok(())
    }

    fn update_labels(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let year = self.year_view_state.get_selected_int()?;
        let day = self.day_view_state.get_selected_int()?;
        if let Some(db) = db {
            if let Some(year) = year {
                self.label_view_state.set_entries(
                    db.get_history_labels(year, day)
                        .map_err(LoreGuiError::LoreCoreError)?,
                );
            }
        } else {
            self.label_view_state = DbColViewState::default();
        }
        self.update_content(db)?;
        Ok(())
    }

    fn update_content(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let label = match self.label_view_state.get_selected() {
            Some(label) => label,
            None => {
                self.current_content = "".to_string();
                return Ok(());
            }
        };
        match db {
            Some(db) => {
                self.current_content = db
                    .get_history_item_content(label)
                    .map_err(LoreGuiError::LoreCoreError)?
            }
            None => self.current_content = String::new(),
        }
        Ok(())
    }
}
