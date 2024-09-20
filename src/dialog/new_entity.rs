use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::sql::lore_database::LoreDatabase;
use lorecore::types::entity::EntityColumn;
use lorecore::types::label::Label;

use crate::app::message_handling::GuiMes;
use crate::errors::LoreGuiError;

use super::{Dialog, DialogMessage};

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

    fn body(&self) -> Element<'_, GuiMes> {
        let label_input = TextInput::new("", self.data.label.to_str())
            .on_input(|i| GuiMes::DialogUpdate(DialogMessage::LabelUpd(i.into())));
        let name_input = TextInput::new("", &self.data.name)
            .on_input(|s| GuiMes::DialogUpdate(DialogMessage::NameUpd(s)));
        let category_input = TextInput::new("", &self.data.category)
            .on_input(|s| GuiMes::DialogUpdate(DialogMessage::CategoryUpd(s)));
        let submit_button = Button::new(Text::new("Create")).on_press(GuiMes::DialogSubmit);
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

    fn update(&mut self, message: DialogMessage) {
        match message {
            DialogMessage::LabelUpd(label) => {
                self.data.label = label;
            }
            DialogMessage::CategoryUpd(ent_type) => {
                self.data.category = ent_type;
            }
            DialogMessage::NameUpd(name) => {
                self.data.name = name;
            }
            _ => (),
        }
    }

    fn submit(&self) -> GuiMes {
        GuiMes::NewEntity(self.data.to_owned())
    }
}
