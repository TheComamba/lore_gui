#![warn(clippy::unwrap_used)]

use std::path::PathBuf;

use app::SqlGui;
use iced::{window, Size};

use crate::user_preferences::store_database_path;

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

const APP_TITLE: &str = "Lore GUI";

fn main() -> iced::Result {
    if let Some(arg1) = std::env::args().nth(1) {
        let path = PathBuf::from(&arg1);
        if path.exists() && path.is_file() {
            if let Err(err) = store_database_path(path) {
                eprintln!(
                    "Could not store database path from command line argument:\n{}",
                    err
                );
            }
        } else {
            eprintln!("No such file: {}", arg1);
        }
    }

    let window_settings = window::Settings {
        size: (Size {
            width: 1820.,
            height: 980.,
        }),
        ..Default::default()
    };
    iced::application(APP_TITLE, SqlGui::update, SqlGui::view)
        .antialiasing(true)
        .window(window_settings)
        .run()
}

#[cfg(test)]
mod tests {
    use lorecore::{sql::lore_database::LoreDatabase, timestamp::current_timestamp, types::*};
    use tempfile::NamedTempFile;

    static UNIQUIFIER: &str = "please_avoid_any_accidental_collisions";

    fn temp_database() -> LoreDatabase {
        let temp_path = NamedTempFile::new()
            .unwrap()
            .into_temp_path()
            .as_os_str()
            .into();
        LoreDatabase::open(temp_path).unwrap()
    }

    pub(crate) fn example_labels() -> Vec<Label> {
        vec![
            format!("{}_{}", "label_1", UNIQUIFIER).into(),
            format!("{}_{}", "label_2", UNIQUIFIER).into(),
            format!("{}_{}", "label_3", UNIQUIFIER).into(),
        ]
    }

    pub(crate) fn example_descriptors() -> Vec<(Descriptor, Description)> {
        vec![
            (
                format!("{} {}", "Descriptor 1", UNIQUIFIER).into(),
                format!("{} {}\n", "Description 1", UNIQUIFIER).into(),
            ),
            (
                format!("{} {}", "Descriptor 2", UNIQUIFIER).into(),
                format!("{} {}\n", "Description 2", UNIQUIFIER).into(),
            ),
            (
                format!("{} {}", "Descriptor 3", UNIQUIFIER).into(),
                format!("{} {}\n", "Description 3", UNIQUIFIER).into(),
            ),
        ]
    }

    pub(crate) fn example_years() -> Vec<Year> {
        vec![0.into(), (-22).into(), 333.into()]
    }

    pub(crate) fn example_days() -> Vec<Day> {
        vec![Day::NONE, 11.into(), 222.into()]
    }

    pub(crate) fn example_history_content(year: Year, day: Day) -> HistoryItemContent {
        format!("{}-{}\n", year, day).into()
    }

    pub(crate) fn example_role(parent: &Parent, child: &Child) -> Role {
        format!("{}-{}", parent, child).into()
    }

    pub(crate) fn example_database() -> LoreDatabase {
        let db = temp_database();
        let labels = example_labels();
        let descriptors = example_descriptors();
        let years = example_years();
        let days = example_days();

        let mut columns = Vec::new();
        for label in labels.iter() {
            for descriptor in descriptors.iter() {
                columns.push(EntityColumn {
                    label: label.clone(),
                    descriptor: descriptor.0.clone(),
                    description: descriptor.1.clone(),
                });
            }
        }
        db.write_entity_columns(columns).unwrap();

        let mut columns = Vec::new();
        for year in years.iter() {
            for day in days.iter() {
                let timestamp = current_timestamp();
                let content = example_history_content(year.clone(), day.clone());
                let properties = HistoryItemProperties::none();
                columns.push(HistoryItem {
                    timestamp,
                    year: year.clone(),
                    day: day.clone(),
                    content,
                    properties,
                });
            }
        }
        db.write_history_items(columns).unwrap();

        let mut columns = Vec::new();
        for parent in labels.iter() {
            for child in labels.iter() {
                if parent == child {
                    continue;
                }
                let parent = parent.to_str().into();
                let child = child.to_str().into();
                let role = example_role(&parent, &child);
                columns.push(EntityRelationship {
                    parent,
                    child,
                    role,
                });
            }
        }
        db.write_relationships(columns).unwrap();

        db
    }
}
