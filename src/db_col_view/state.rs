use lorecore::sql::search_text::SqlSearchText;

use crate::errors::LoreGuiError;

#[derive(Debug, Clone)]
pub(crate) struct DbColViewState {
    search_text: String,
    entries: Vec<String>,
    selected_entry: Option<String>,
    visible_entries: Vec<String>,
}

impl DbColViewState {
    pub(crate) fn new(entries: Vec<String>) -> Self {
        let mut state = DbColViewState {
            search_text: String::new(),
            entries: vec![],
            selected_entry: None,
            visible_entries: vec![],
        };
        state.set_entries(entries);
        state
    }

    pub(crate) fn get_selected_int(&self) -> Result<Option<i32>, LoreGuiError> {
        let year = match self.selected_entry.as_ref() {
            Some(year) => year
                .parse::<i32>()
                .map_err(|e| LoreGuiError::InputError(e.to_string()))?,
            None => return Ok(None),
        };
        Ok(Some(year))
    }

    pub(crate) fn set_entries(&mut self, mut entries: Vec<String>) {
        if entries.is_empty() {
            return;
        }
        if !entries.contains(&String::new()) {
            entries.push(String::new());
        }
        entries.sort();
        entries.dedup();
        self.entries = entries;
        self.set_visible_entries();
    }

    pub(crate) fn set_selected(&mut self, entry: String) {
        if entry.is_empty() {
            self.selected_entry = None;
        } else {
            self.selected_entry = Some(entry);
        }
    }

    pub(crate) fn set_selected_none(&mut self) {
        self.selected_entry = None;
    }

    pub(crate) fn get_selected(&self) -> &Option<String> {
        &self.selected_entry
    }

    pub(crate) fn set_search_text(&mut self, text: String) {
        self.search_text = text;
        self.set_visible_entries();
    }

    pub(crate) fn get_search_text(&self) -> &str {
        &self.search_text
    }

    pub(crate) fn get_sql_search_text(&self) -> SqlSearchText {
        SqlSearchText::new(&self.search_text)
    }

    pub(crate) fn get_visible_entries(&self) -> &Vec<String> {
        &self.visible_entries
    }

    fn set_visible_entries(&mut self) {
        self.visible_entries = match self.search_text.is_empty() {
            true => self.entries.clone(),
            false => {
                let mut visible = vec![String::new()];
                for entry in self.entries.iter() {
                    if entry.contains(&self.search_text) {
                        visible.push(entry.clone());
                    }
                }
                visible
            }
        }
    }
}

impl Default for DbColViewState {
    fn default() -> Self {
        Self::new(vec![])
    }
}
