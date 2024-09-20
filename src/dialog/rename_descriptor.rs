use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{descriptor::Descriptor, label::Label},
};

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::{Dialog, DialogMessage};

#[derive(Debug, Clone)]
pub(crate) struct RenameDescriptorDialog {
    data: RenameDescriptorData,
}

impl RenameDescriptorDialog {
    pub(crate) fn new(data: RenameDescriptorData) -> Self {
        RenameDescriptorDialog { data }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RenameDescriptorData {
    pub(self) label: Label,
    pub(self) old_descriptor: Descriptor,
    pub(self) new_descriptor: Descriptor,
}

impl RenameDescriptorData {
    pub(crate) fn new(label: Label, old_descriptor: Descriptor) -> Self {
        RenameDescriptorData {
            label,
            new_descriptor: old_descriptor.clone(),
            old_descriptor,
        }
    }

    pub(crate) fn update_descriptor_in_database(
        self,
        db: &LoreDatabase,
    ) -> Result<(), LoreGuiError> {
        if self.old_descriptor.to_str().is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot rename descriptor with empty descriptor.".to_string(),
            ));
        }
        if self.new_descriptor.to_str().is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot rename descriptor with empty new descriptor.".to_string(),
            ));
        }

        db.change_entity_descriptor((&self.label, self.old_descriptor), &self.new_descriptor)?;
        Ok(())
    }

    pub(crate) fn get_descriptor(&self) -> &Descriptor {
        &self.old_descriptor
    }
}

impl Dialog for RenameDescriptorDialog {
    fn header(&self) -> String {
        format!(
            "Rename descriptor {} of entity {}",
            self.data.old_descriptor, self.data.label
        )
    }

    fn body(&self) -> Element<'_, GuiMes> {
        let new_descriptor_input = TextInput::new("", self.data.new_descriptor.to_str())
            .on_input(|i| GuiMes::DialogUpdate(DialogMessage::NewDescriptorUpd(i.into())));
        let submit_button = Button::new(Text::new("Update")).on_press(GuiMes::DialogSubmit);
        Column::new()
            .push(Text::new("New Descriptor"))
            .push(new_descriptor_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn update(&mut self, message: super::DialogMessage) {
        if let DialogMessage::NewDescriptorUpd(new_descriptor) = message {
            self.data.new_descriptor = new_descriptor;
        }
    }

    fn submit(&self) -> GuiMes {
        GuiMes::RenameDescriptor(self.data.to_owned())
    }
}
