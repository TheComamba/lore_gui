#![warn(clippy::unwrap_used)]

use app::SqlGui;

mod app;
mod db_col_view;
mod dialog;
mod editor;
mod entity_view;
mod errors;
mod file_dialogs;
mod history_view;
mod relationship_view;
mod style;
mod user_preferences;

const APP_TITLE: &str = "Lore SQL GUI";

fn main() -> iced::Result {
    iced::application(APP_TITLE, SqlGui::update, SqlGui::view).run()
}

#[cfg(test)]
mod tests {
    use lorecore::sql::lore_database::LoreDatabase;
    use tempfile::NamedTempFile;

    pub(crate) fn temp_database() -> LoreDatabase {
        let temp_path = NamedTempFile::new()
            .unwrap()
            .into_temp_path()
            .as_os_str()
            .into();
        LoreDatabase::open(temp_path).unwrap()
    }
}
