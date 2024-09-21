use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::sql::lore_database::LoreDatabase;
use lorecore::types::entity::EntityColumn;
use lorecore::types::label::Label;

use crate::app::message_handling::GuiMessage;
use crate::errors::LoreGuiError;

use super::{Dialog, DialogUpdate};

#[derive(Debug, Clone)]
pub(crate) struct NewEntityDialog {
    data: NewEntityData,
}

impl NewEntityDialog {
    pub(crate) fn new() -> Self {
        NewEntityDialog {
            data: NewEntityData::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NewEntityData {
    pub(self) label: Label,
    pub(self) name: String,
    pub(self) category: String,
}

impl NewEntityData {
    pub(crate) fn new() -> Self {
        NewEntityData {
            label: "".into(),
            name: String::new(),
            category: String::new(),
        }
    }

    pub(crate) fn write_to_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        if self.label.to_str().is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot create entity with empty label.".to_string(),
            ));
        }
        if self.name.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot create entity with empty name.".to_string(),
            ));
        }
        if self.category.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot create entity with empty category.".to_string(),
            ));
        }

        let category_descriptor = "_category".into();
        let name_descriptor = "_name".into();

        let name_col = EntityColumn {
            label: self.label.clone(),
            descriptor: name_descriptor,
            description: self.name.into(),
        };
        let category_col = EntityColumn {
            label: self.label,
            descriptor: category_descriptor,
            description: self.category.into(),
        };

        db.write_entity_columns(vec![name_col])?;
        db.write_entity_columns(vec![category_col])?;

        Ok(())
    }

    pub(crate) fn get_label(&self) -> &Label {
        &self.label
    }
}

impl Dialog for NewEntityDialog {
    fn header(&self) -> String {
        "Create new entity".to_string()
    }

    fn body(&self) -> Element<'_, GuiMessage> {
        let label_input = TextInput::new("", self.data.label.to_str())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Label(i.into())));
        let name_input = TextInput::new("", &self.data.name)
            .on_input(|s| GuiMessage::DialogUpdate(DialogUpdate::Name(s)));
        let category_input = TextInput::new("", &self.data.category)
            .on_input(|s| GuiMessage::DialogUpdate(DialogUpdate::Category(s)));
        let submit_button = Button::new(Text::new("Create")).on_press(GuiMessage::DialogSubmit);
        Column::new()
            .push(Text::new("Label:"))
            .push(label_input)
            .push(Text::new("Name"))
            .push(name_input)
            .push(Text::new("Category:"))
            .push(category_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn update(&mut self, message: DialogUpdate) {
        match message {
            DialogUpdate::Label(label) => {
                self.data.label = label;
            }
            DialogUpdate::Category(ent_type) => {
                self.data.category = ent_type;
            }
            DialogUpdate::Name(name) => {
                self.data.name = name;
            }
            _ => (),
        }
    }

    fn submit(&self) -> GuiMessage {
        GuiMessage::NewEntity(self.data.to_owned())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use lorecore::types::label::Label;

    use crate::dialog::new_entity::NewEntityData;

    pub(crate) fn example_new_entity_data() -> NewEntityData {
        let label = Label::from("new_entity");
        let name = String::from("New Entity");
        let category = String::from("Some Category");
        NewEntityData {
            label,
            name,
            category,
        }
    }
}
