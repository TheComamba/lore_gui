use iced::{
    widget::{component, Button, Column, Component, Text, TextInput},
    Element,
};
use lorecore::sql::lore_database::LoreDatabase;

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::Dialog;

#[derive(Debug, Clone)]
pub(crate) struct RelabelEntityDialog {
    data: RelabelEntityData,
}

impl RelabelEntityDialog {
    pub(crate) fn new(old_label: String) -> Self {
        RelabelEntityDialog {
            data: RelabelEntityData::new(old_label),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RelabelEntityData {
    pub(self) old_label: String,
    pub(self) new_label: String,
}

impl RelabelEntityData {
    pub(crate) fn new(old_label: String) -> Self {
        RelabelEntityData {
            old_label,
            new_label: String::new(),
        }
    }

    pub(crate) fn update_label_in_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        if self.old_label.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot relabel entity with empty label.".to_string(),
            ));
        }
        if self.new_label.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot relabel entity with empty new label.".to_string(),
            ));
        }

        todo!();
        //db.update_entity_label(&self.label, &self.new_label)?;

        Ok(())
    }

    pub(crate) fn get_label(&self) -> &str {
        &self.old_label
    }
}

impl Dialog for RelabelEntityDialog {
    fn header(&self) -> String {
        "Relabel entity".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }
}

impl Component<GuiMes> for RelabelEntityDialog {
    type State = ();

    type Event = RelabelEntityMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            RelabelEntityMes::LabelUpd(label) => {
                self.data.old_label = label;
                None
            }
            RelabelEntityMes::NewLabelUpd(new_label) => {
                self.data.new_label = new_label;
                None
            }
            RelabelEntityMes::Submit => Some(GuiMes::RelabelEntity(self.data.to_owned())),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let label_input =
            TextInput::new("", &self.data.old_label).on_input(RelabelEntityMes::LabelUpd);
        let new_label_input =
            TextInput::new("", &self.data.new_label).on_input(RelabelEntityMes::NewLabelUpd);
        let submit_button = Button::new(Text::new("Update")).on_press(RelabelEntityMes::Submit);
        Column::new()
            .push(Text::new("Current Label:"))
            .push(label_input)
            .push(Text::new("New Label"))
            .push(new_label_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RelabelEntityMes {
    LabelUpd(String),
    NewLabelUpd(String),
    Submit,
}
