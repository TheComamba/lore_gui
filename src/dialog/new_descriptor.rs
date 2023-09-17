use iced::{
    widget::{component, Button, Column, Component, Text, TextInput},
    Element, Renderer,
};
use lorecore::sql::lore_database::LoreDatabase;

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::Dialog;

#[derive(Debug, Clone)]
pub(crate) struct NewDescriptorDialog {
    data: NewDescriptorData,
}

impl NewDescriptorDialog {
    pub(crate) fn new(label: String) -> Self {
        NewDescriptorDialog {
            data: NewDescriptorData::new(label),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NewDescriptorData {
    pub(self) label: String,
    pub(self) descriptor: String,
}

impl NewDescriptorData {
    pub(crate) fn new(label: String) -> Self {
        NewDescriptorData {
            label,
            descriptor: String::new(),
        }
    }

    pub(crate) fn write_to_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        if self.descriptor.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot create empty descriptor.".to_string(),
            ));
        }

        let col = lorecore::sql::entity::EntityColumn {
            label: self.label,
            descriptor: self.descriptor,
            description: None,
        };
        db.write_entity_columns(vec![col])
            .map_err(LoreGuiError::LoreCoreError)
    }

    pub(crate) fn get_descriptor(&self) -> &str {
        &self.descriptor
    }
}

impl Dialog for NewDescriptorDialog {
    fn header(&self) -> String {
        "New Descriptor".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }
}

impl Component<GuiMes, Renderer> for NewDescriptorDialog {
    type State = ();

    type Event = NewDescriptorMessage;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let descriptor_input =
            TextInput::new("", &self.data.label).on_input(NewDescriptorMessage::DescriptorUpd);
        let submit_button = Button::new(Text::new("Create")).on_press(NewDescriptorMessage::Submit);
        Column::new()
            .push(Text::new("Descriptor:"))
            .push(descriptor_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewDescriptorMessage {
    DescriptorUpd(String),
    Submit,
}
