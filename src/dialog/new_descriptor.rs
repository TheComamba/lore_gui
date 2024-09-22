use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{description::Description, descriptor::Descriptor, entity::EntityColumn, label::Label},
};

use crate::{app::message_handling::GuiMessage, errors::LoreGuiError};

use super::{Dialog, DialogUpdate};

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

    #[cfg(test)]
    pub(crate) fn label(&self) -> &Label {
        &self.label
    }

    pub(crate) fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }

    #[cfg(test)]
    pub(crate) fn description(&self) -> &Description {
        &self.description
    }

    #[cfg(test)]
    pub(crate) fn set_descriptor(&mut self, descriptor: Descriptor) {
        self.descriptor = descriptor;
    }
}

impl Dialog for NewDescriptorDialog {
    fn header(&self) -> String {
        "New Descriptor".to_string()
    }

    fn body(&self) -> Element<'_, GuiMessage> {
        let descriptor_input = TextInput::new("", self.data.descriptor.to_str())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Descriptor(i.into())));
        let description_input = TextInput::new("", self.data.description.to_str())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Description(i.into())));
        let submit_button = Button::new(Text::new("Create")).on_press(GuiMessage::DialogSubmit);
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

    fn update(&mut self, message: DialogUpdate) {
        match message {
            DialogUpdate::Descriptor(descriptor) => {
                self.data.descriptor = descriptor;
            }
            DialogUpdate::Description(description) => {
                self.data.description = description;
            }
            _ => (),
        }
    }

    fn submit(&self) -> GuiMessage {
        GuiMessage::NewDescriptor(self.data.to_owned())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::dialog::new_descriptor::NewDescriptorData;

    pub(crate) fn example_new_descriptor_data() -> NewDescriptorData {
        NewDescriptorData {
            label: "test".into(),
            descriptor: "test_descriptor".into(),
            description: "test_description".into(),
        }
    }
}
