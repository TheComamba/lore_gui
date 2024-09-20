use iced::{
    widget::{component, Button, Column, Component, Text, TextInput},
    Element,
};
use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{description::Description, descriptor::Descriptor, entity::EntityColumn, label::Label},
};

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::{Dialog, DialogMessage};

#[derive(Debug, Clone)]
pub(crate) struct NewDescriptorDialog {
    data: NewDescriptorData,
}

impl NewDescriptorDialog {
    pub(crate) fn new(label: Label) -> Self {
        NewDescriptorDialog {
            data: NewDescriptorData::new(label),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NewDescriptorData {
    pub(self) label: Label,
    pub(self) descriptor: Descriptor,
    pub(self) description: Description,
}

impl NewDescriptorData {
    pub(crate) fn new(label: Label) -> Self {
        NewDescriptorData {
            label,
            descriptor: "".into(),
            description: "".into(),
        }
    }

    pub(crate) fn write_to_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        if self.descriptor.to_str().is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot create empty descriptor.".to_string(),
            ));
        }

        let col = EntityColumn {
            label: self.label,
            descriptor: self.descriptor,
            description: self.description,
        };
        db.write_entity_columns(vec![col])
            .map_err(LoreGuiError::from)
    }

    pub(crate) fn get_descriptor(&self) -> &Descriptor {
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

    fn update(&mut self, message: DialogMessage) {
        match message {
            DialogMessage::DescriptorUpd(descriptor) => {
                self.data.descriptor = descriptor;
            }
            DialogMessage::DescriptionUpd(description) => {
                self.data.description = description;
            }
            _ => (),
        }
    }

    fn submit(&self) -> GuiMes {
        GuiMes::NewDescriptor(self.data.to_owned())
    }
}

impl Component<GuiMes> for NewDescriptorDialog {
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

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let descriptor_input = TextInput::new("", self.data.descriptor.to_str())
            .on_input(|i| NewDescriptorMessage::DescriptorUpd(i.into()));
        let description_input = TextInput::new("", self.data.description.to_str())
            .on_input(|i| NewDescriptorMessage::DescriptionUpd(i.into()));
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
    DescriptorUpd(Descriptor),
    DescriptionUpd(Description),
    Submit,
}
