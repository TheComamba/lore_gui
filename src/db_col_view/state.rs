use crate::errors::LoreGuiError;

#[derive(Debug, Clone)]
pub(crate) struct DbColViewState {
    search_text: String,
    entries: Vec<String>,
    selected_entry: Option<String>,
}

impl DbColViewState {
    pub(crate) fn new(entries: Vec<String>) -> Self {
        let mut state = DbColViewState {
            search_text: String::new(),
            entries: vec![],
            selected_entry: None,
        };
        state.set_entries(entries);
        state
    }

    pub(crate) fn get_selected_as<T>(&self) -> Result<Option<T>, LoreGuiError>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        let year = match self.selected_entry.as_ref() {
            Some(year) => year
                .parse::<T>()
                .map_err(|e| LoreGuiError::InputError(e.to_string()))?,
            None => return Ok(None),
        };
        Ok(Some(year))
    }

    pub(crate) fn set_entries(&mut self, mut entries: Vec<String>) {
        if !entries.contains(&String::new()) {
            entries.push(String::new());
        }
        entries.sort();
        entries.dedup();
        self.entries = entries;
    }

    pub(crate) fn get_entries(&self) -> &Vec<String> {
        &self.entries
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
    }

    pub(crate) fn get_search_text(&self) -> Option<&str> {
        if self.search_text.is_empty() {
            None
        } else {
            Some(&self.search_text)
        }
    }
}

impl Default for DbColViewState {
    fn default() -> Self {
        Self::new(vec![])
    }
}
