use lorecore::{sql::lore_database::LoreDatabase, types::*};

use crate::{
    db_col_view::{entry::DbColViewEntry, ColViewMes},
    dialog::{
        confirmation::ConfirmationDialog,
        new_history_item::{NewHistoryData, NewHistoryDialog},
        redate_history::{RedateHistoryData, RedateHistoryDialog},
    },
    editor::EditorState,
    errors::LoreGuiError,
    history_view::{HistoryViewMessage, HistoryViewState},
};

use super::{message_handling::GuiMessage, SqlGui};

impl SqlGui {
    pub(super) fn update_history_view(
        &mut self,
        event: HistoryViewMessage,
    ) -> Result<(), LoreGuiError> {
        match event {
            HistoryViewMessage::NewHistoryItem => {
                self.dialog = Some(Box::new(NewHistoryDialog::new()))
            }
            HistoryViewMessage::RedateHistoryItem(data) => {
                self.dialog = Some(Box::new(RedateHistoryDialog::new(data)))
            }
            HistoryViewMessage::DeleteHistoryItem(timestamp) => {
                let message = format!("Do you really want to delete {}?", timestamp);
                let on_confirm = GuiMessage::DeleteHistoryItem(timestamp);
                self.dialog = Some(Box::new(ConfirmationDialog::new(message, on_confirm)))
            }
            HistoryViewMessage::YearViewUpdate(event) => self.update_year_view(event)?,
            HistoryViewMessage::DayViewUpdate(event) => self.update_day_view(event)?,
            HistoryViewMessage::HistoryTimestampViewUpdate(event) => {
                self.update_timestamp_view(event)?
            }
            HistoryViewMessage::ContentUpdate(action) => {
                self.history_view_state.current_content.perform(action)
            }
            HistoryViewMessage::ContentDiscard => self.history_view_state.current_content.reset(),
            HistoryViewMessage::ContentSave => {
                let db = self
                    .lore_database
                    .as_ref()
                    .ok_or(LoreGuiError::NoDatabase)?;
                let timestamp = match self.get_selected_timestamp() {
                    Some(t) => t,
                    None => return Ok(()),
                };
                let content = HistoryItemContent::from(self.get_history_text());
                db.change_history_item_content(timestamp, &content)?;
                self.history_view_state.current_content.saved();
            }
        };
        self.history_view_state.update(&self.lore_database)?;
        Ok(())
    }

    pub(super) fn update_year_view(&mut self, event: ColViewMes<Year>) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.year_view_state.set_search_text(text);
            }
            ColViewMes::Selected(_index, year) => {
                state.set_selected_year(year.0);
                state.set_selected_day(None);
                state.set_selected_timestamp(None);
            }
        };
        self.history_view_state.update(&self.lore_database)?;
        Ok(())
    }

    pub(super) fn update_day_view(&mut self, event: ColViewMes<Day>) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.day_view_state.set_search_text(text);
            }
            ColViewMes::Selected(_index, day) => {
                state.set_selected_day(day.0);
                state.set_selected_timestamp(None);
            }
        };
        self.history_view_state.update(&self.lore_database)?;
        Ok(())
    }

    pub(super) fn update_timestamp_view(
        &mut self,
        event: ColViewMes<Timestamp>,
    ) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.timestamp_view_state.set_search_text(text);
            }
            ColViewMes::Selected(_index, timestamp) => {
                state.set_selected_timestamp(timestamp.0);
            }
        };
        self.history_view_state.update(&self.lore_database)?;
        Ok(())
    }

    pub(super) fn write_new_history(&mut self, data: NewHistoryData) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let year = data.year().clone();
        let day = data.day().clone();
        let timestamp = data.timestamp().clone();
        data.write_to_database(db)?;
        self.set_selected_year(Some(year));
        self.set_selected_day(Some(day));
        self.set_selected_timestamp(Some(timestamp));
        Ok(())
    }

    pub(super) fn redate_history_item(
        &mut self,
        data: RedateHistoryData,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let year = data.new_year().clone();
        let day = data.new_day().clone();
        let timestamp = data.timestamp();
        data.update_date_in_database(db)?;
        self.set_selected_year(Some(year));
        self.set_selected_day(Some(day));
        self.set_selected_timestamp(Some(timestamp));
        Ok(())
    }

    pub(super) fn delete_history_item(&mut self, timestamp: Timestamp) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        db.delete_history_item(timestamp)?;
        self.set_selected_year(None);
        self.set_selected_day(None);
        self.set_selected_timestamp(None);
        self.set_history_text("");
        Ok(())
    }
}

impl HistoryViewState {
    pub(super) fn reset_selections(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<(), LoreGuiError> {
        self.year_view_state.set_selected(DbColViewEntry::NONE);
        self.day_view_state.set_selected(DbColViewEntry::NONE);
        self.timestamp_view_state.set_selected(DbColViewEntry::NONE);
        self.current_content = EditorState::default();
        self.update_years(db)?;
        Ok(())
    }

    pub(super) fn update(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        self.update_years(db)?;
        self.update_days(db)?;
        self.update_timestamps(db)?;
        self.update_content(db)?;
        Ok(())
    }

    fn update_years(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let years = self
            .get_current_years(db)?
            .into_iter()
            .map(|y| DbColViewEntry(Some(y)))
            .collect();
        self.year_view_state.set_entries(years);
        Ok(())
    }

    fn update_days(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let days = self
            .get_current_days(db)?
            .into_iter()
            .map(|d| DbColViewEntry(Some(d)))
            .collect();
        self.day_view_state.set_entries(days);
        Ok(())
    }

    fn update_timestamps(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let timestamps = self
            .get_current_timestamps(db)?
            .into_iter()
            .map(|t| DbColViewEntry(Some(t)))
            .collect();
        self.timestamp_view_state.set_entries(timestamps);
        Ok(())
    }

    fn update_content(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let content = self.get_current_content(db)?;
        self.current_content = EditorState::new(content.to_str());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use lorecore::timestamp::current_timestamp;

    use super::*;

    use crate::tests::{example_database, example_days, example_history_content, example_years};

    #[test]
    fn selecting_year_deselects_day_and_timestamp() {
        let mut gui = SqlGui {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let years = example_years();
        let days = example_days();
        gui.set_selected_year(Some(years[0].clone()));
        gui.set_selected_day(Some(days[0].clone()));
        gui.set_selected_timestamp(Some(current_timestamp()));
        let content = example_history_content(years[0], days[0]);
        gui.set_history_text(content.to_str());

        let new_year = years[1].clone();
        let event = ColViewMes::Selected(1, DbColViewEntry(Some(new_year.clone())));
        gui.update_year_view(event).unwrap();

        assert_eq!(gui.get_selected_year(), Some(new_year));
        assert_eq!(gui.get_selected_day(), None);
        assert_eq!(gui.get_selected_timestamp(), None);
        assert_eq!(gui.get_history_text(), "\n");
    }
}
