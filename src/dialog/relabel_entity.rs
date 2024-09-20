use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{sql::lore_database::LoreDatabase, types::label::Label};

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::{Dialog, DialogMessage};

#[derive(Debug, Clone)]
pub(crate) struct RelabelEntityDialog {
    data: RelabelEntityData,
}

impl RelabelEntityDialog {
    pub(crate) fn new(data: RelabelEntityData) -> Self {
        RelabelEntityDialog { data }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RelabelEntityData {
    pub(self) old_label: Label,
    pub(self) new_label: Label,
}

impl RelabelEntityData {
    pub(crate) fn new(old_label: Label) -> Self {
        RelabelEntityData {
            new_label: old_label.clone(),
            old_label,
        }
    }

    pub(crate) fn update_label_in_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        if self.old_label.to_str().is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot relabel entity with empty label.".to_string(),
            ));
        }
        if self.new_label.to_str().is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot relabel entity with empty new label.".to_string(),
            ));
        }

        db.relabel_entity(&self.old_label, &self.new_label)?;
        Ok(())
    }

    pub(crate) fn get_label(&self) -> &Label {
        &self.new_label
    }
}

impl Dialog for RelabelEntityDialog {
    fn header(&self) -> String {
        format!("Relabel entity: {}", self.data.old_label)
    }

    fn body<'a>(&'a self) -> Element<'a, GuiMes> {
        let new_label_input = TextInput::new("", self.data.new_label.to_str())
            .on_input(|i| GuiMes::DialogUpdate(DialogMessage::NewLabelUpd(i.into())));
        let submit_button = Button::new(Text::new("Update")).on_press(GuiMes::DialogSubmit);
        Column::new()
            .push(Text::new("New Label"))
            .push(new_label_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn update(&mut self, message: DialogMessage) {
        match message {
            DialogMessage::NewLabelUpd(new_label) => {
                self.data.new_label = new_label;
            }
            _ => (),
        }
    }

    fn submit(&self) -> GuiMes {
        GuiMes::RelabelEntity(self.data.to_owned())
    }
}
