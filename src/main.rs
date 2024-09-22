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

    fn example_labels() -> Vec<Label> {
        vec![
            format!("{}_{}", "label_1", UNIQUIFIER).into(),
            format!("{}_{}", "label_2", UNIQUIFIER).into(),
            format!("{}_{}", "label_3", UNIQUIFIER).into(),
        ]
    }

    fn example_descriptors() -> Vec<(Descriptor, Description)> {
        vec![
            (
                format!("{} {}", "Descriptor 1", UNIQUIFIER).into(),
                format!("{} {}", "Description 1", UNIQUIFIER).into(),
            ),
            (
                format!("{} {}", "Descriptor 2", UNIQUIFIER).into(),
                format!("{} {}", "Description 2", UNIQUIFIER).into(),
            ),
            (
                format!("{} {}", "Descriptor 3", UNIQUIFIER).into(),
                format!("{} {}", "Description 3", UNIQUIFIER).into(),
            ),
        ]
    }

    fn example_years() -> Vec<Year> {
        vec![0.into(), (-22).into(), 333.into()]
    }

    fn example_days() -> Vec<Day> {
        vec![Day::NONE, 11.into(), 222.into()]
    }

    fn example_history_content(year: Year, day: Day) -> HistoryItemContent {
        format!("{}-{}", year, day).into()
    }

    fn example_role(parent: &Parent, child: &Child) -> Role {
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
