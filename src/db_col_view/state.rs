use crate::errors::LoreGuiError;

use super::entry::DbColViewEntry;

#[derive(Debug, Clone)]
pub(crate) struct DbColViewState<E: DbColViewEntry> {
    search_text: String,
    entries: Vec<E>,
    selected_entry: Option<E>,
}

impl<E: DbColViewEntry> DbColViewState<E> {
    pub(crate) fn new(entries: Vec<E>) -> Self {
        let mut state = DbColViewState {
            search_text: String::new(),
            entries: vec![],
            selected_entry: None,
        };
        state.set_entries(entries);
        state
    }

    pub(crate) fn set_entries(&mut self, mut entries: Vec<E>) {
        self.entries = entries;
    }

    pub(super) fn get_entries(&self) -> Vec<Option<E>> {
        //TODO: Fix this.
        // if !entries.contains(&String::new()) {
        //     entries.insert(0, String::new());
        // }
        self.entries.iter().map(|e| Some(*e.clone())).collect()
    }

    pub(crate) fn set_selected(&mut self, entry: Option<E>) {
        self.selected_entry = entry;
    }

    pub(crate) fn get_selected(&self) -> &Option<E> {
        &self.selected_entry
    }

    pub(crate) fn set_search_text(&mut self, text: String) {
        self.search_text = text;
    }

    pub(crate) fn get_search_text(&self) -> Option<&str> {
        if self.search_text.is_empty() {
            None
        } else {
            Some(&self.search_text)
        }
    }

    pub(crate) fn get_search_int(&self) -> Result<Option<i32>, LoreGuiError> {
        let search_text = self.get_search_text().map(|t| t.parse::<i32>());
        let search_int = match search_text {
            Some(Ok(i)) => Some(i),
            Some(Err(e)) => return Err(LoreGuiError::InputError(e.to_string())),
            None => None,
        };
        Ok(search_int)
    }
}

impl<E: DbColViewEntry> Default for DbColViewState<E> {
    fn default() -> Self {
        Self::new(vec![])
    }
}
