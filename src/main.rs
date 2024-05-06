#![warn(clippy::unwrap_used)]

use app::SqlGui;
use iced::{Sandbox, Settings};

mod app;
mod db_col_view;
mod dialog;
mod entity_view;
mod errors;
mod file_dialogs;
mod history_view;
mod relationship_view;
mod style;
mod user_preferences;

const APP_TITLE: &str = "Lore SQL GUI";

fn main() -> iced::Result {
    SqlGui::run(Settings::default())
}
