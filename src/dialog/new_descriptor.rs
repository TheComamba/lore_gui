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
    pub(self) description: String,
}

impl NewDescriptorData {
    pub(crate) fn new(label: String) -> Self {
        NewDescriptorData {
            label,
            descriptor: String::new(),
            description: String::new(),
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
            description: Some(self.description),
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
        match event {
            NewDescriptorMessage::DescriptorUpd(descriptor) => {
                self.data.descriptor = descriptor;
                None
            }
            NewDescriptorMessage::DescriptionUpd(description) => {
                self.data.description = description;
                None
            }
            NewDescriptorMessage::Submit => Some(GuiMes::NewDescriptor(self.data.to_owned())),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let descriptor_input =
            TextInput::new("", &self.data.descriptor).on_input(NewDescriptorMessage::DescriptorUpd);
        let description_input = TextInput::new("", &self.data.description)
            .on_input(NewDescriptorMessage::DescriptionUpd);
        let submit_button = Button::new(Text::new("Create")).on_press(NewDescriptorMessage::Submit);
        Column::new()
            .push(Text::new("Descriptor:"))
            .push(descriptor_input)
            .push(Text::new("Description:"))
            .push(description_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewDescriptorMessage {
    DescriptorUpd(String),
    DescriptionUpd(String),
    Submit,
}
