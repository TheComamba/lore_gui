use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{sql::lore_database::LoreDatabase, timestamp::current_timestamp, types::*};

use crate::{app::message_handling::GuiMessage, errors::LoreGuiError};

use super::{Dialog, DialogUpdate};

#[derive(Clone, Debug)]
pub(crate) struct NewHistoryDialog {
    data: NewHistoryData,
}

impl NewHistoryDialog {
    pub(crate) fn new() -> Self {
        NewHistoryDialog {
            data: NewHistoryData {
                year: 0.into(),
                day: Day::NONE,
                content: "".into(),
                properties: HistoryItemProperties::none(),
                timestamp: current_timestamp(),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NewHistoryData {
    pub(self) timestamp: Timestamp,
    pub(self) year: Year,
    pub(self) day: Day,
    pub(self) content: HistoryItemContent,
    pub(self) properties: HistoryItemProperties,
}

impl NewHistoryData {
    pub(crate) fn write_to_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        let item = HistoryItem {
            timestamp: self.timestamp,
            year: self.year,
            day: self.day,
            content: self.content,
            properties: self.properties,
        };
        db.write_history_items(vec![item])?;
        Ok(())
    }

    pub(crate) fn year(&self) -> &Year {
        &self.year
    }

    pub(crate) fn day(&self) -> &Day {
        &self.day
    }

    #[cfg(test)]
    pub(crate) fn content(&self) -> &HistoryItemContent {
        &self.content
    }

    pub(crate) fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
}

impl Dialog for NewHistoryDialog {
    fn header(&self) -> String {
        "Create new history item".to_string()
    }

    fn body(&self) -> Element<'_, GuiMessage> {
        let year_input = TextInput::new("", &self.data.year.to_string())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Year(i.try_into())));
        let day_string = format!("{}", self.data.day);
        let day_input = TextInput::new("", &day_string)
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Day(i.try_into())));
        let content_input = TextInput::new("", self.data.content.to_str())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Content(i.into())));
        let submit_button = Button::new("Create").on_press(GuiMessage::DialogSubmit);
        Column::new()
            .push(Text::new("Year:"))
            .push(year_input)
            .push(Text::new("Day:"))
            .push(day_input)
            .push(Text::new("Content:"))
            .push(content_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn update(&mut self, message: DialogUpdate) {
        match message {
            DialogUpdate::Year(Ok(year)) => self.data.year = year,
            DialogUpdate::Day(Ok(day)) => self.data.day = day,
            DialogUpdate::Content(content) => self.data.content = content,
            _ => (),
        }
    }

    fn submit(&self) -> GuiMessage {
        GuiMessage::NewHistoryItem(self.data.clone())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use lorecore::timestamp::current_timestamp;
    use serde_json::json;
    use std::collections::HashMap;

    use super::*;

    pub(crate) fn example_new_history_data() -> NewHistoryData {
        let year = 2021.into();
        let day = Day::from(11);
        let content = HistoryItemContent::from("Example content");
        let mut properties_map = HashMap::new();
        properties_map.insert("key1".to_string(), json!("value1"));
        properties_map.insert("key2".to_string(), json!(42));
        properties_map.insert("key3".to_string(), json!({"nested_key": "nested_value"}));
        let properties = HistoryItemProperties::from(properties_map);
        let timestamp = current_timestamp();
        NewHistoryData {
            year,
            day,
            content,
            properties,
            timestamp,
        }
    }
}
