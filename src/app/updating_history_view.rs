use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{day::Day, history_item_content::HistoryItemContent, timestamp::Timestamp, year::Year},
};

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
                let timestamp = match self
                    .history_view_state
                    .timestamp_view_state
                    .get_selected()
                    .0
                {
                    Some(t) => t,
                    None => return Ok(()),
                };
                let content =
                    HistoryItemContent::from(self.history_view_state.current_content.get_text());
                db.change_history_item_content(timestamp, &content)?;
                self.history_view_state.current_content.save();
            }
        };
        Ok(())
    }

    pub(super) fn update_year_view(&mut self, event: ColViewMes<Year>) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.year_view_state.set_search_text(text);
                state.update_years(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, year) => {
                state.year_view_state.set_selected(year);
                state.day_view_state.set_selected(DbColViewEntry::NONE);
                state.update_days(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_day_view(&mut self, event: ColViewMes<Day>) -> Result<(), LoreGuiError> {
        let state = &mut self.history_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.day_view_state.set_search_text(text);
                state.update_days(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, day) => {
                state.day_view_state.set_selected(day);
                state
                    .timestamp_view_state
                    .set_selected(DbColViewEntry::NONE);
                state.update_timestamps(&self.lore_database)?;
            }
        };
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
                state.update_timestamps(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, timestamp) => {
                state.timestamp_view_state.set_selected(timestamp);
                state.update_content(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn write_new_history(&mut self, data: NewHistoryData) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let year = DbColViewEntry(Some(data.year));
        let day = DbColViewEntry(Some(data.day));
        data.write_to_database(db)?;
        self.update_year_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_year_view(ColViewMes::Selected(0, year))?;
        self.update_day_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_day_view(ColViewMes::Selected(0, day))?;
        self.dialog = None;
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
        data.update_date_in_database(db)?;
        self.dialog = None;
        Ok(())
    }

    pub(super) fn delete_history_item(&mut self, timestamp: Timestamp) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        db.delete_history_item(timestamp)?;
        self.dialog = None;
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

    fn update_years(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let years = self
            .get_current_years(db)?
            .into_iter()
            .map(|y| DbColViewEntry(Some(y)))
            .collect();
        self.year_view_state.set_entries(years);
        self.update_days(db)?;
        Ok(())
    }

    fn update_days(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let days = self
            .get_current_days(db)?
            .into_iter()
            .map(|d| DbColViewEntry(Some(d)))
            .collect();
        self.day_view_state.set_entries(days);
        self.update_timestamps(db)?;
        Ok(())
    }

    fn update_timestamps(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let timestamps = self
            .get_current_timestamps(db)?
            .into_iter()
            .map(|t| DbColViewEntry(Some(t)))
            .collect();
        self.timestamp_view_state.set_entries(timestamps);
        self.update_content(db)?;
        Ok(())
    }

    fn update_content(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let content = self.get_current_content(db)?;
        self.current_content = EditorState::new(content.to_str());
        Ok(())
    }
}
