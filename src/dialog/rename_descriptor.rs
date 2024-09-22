use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{descriptor::Descriptor, label::Label},
};

use crate::{app::message_handling::GuiMessage, errors::LoreGuiError};

use super::{Dialog, DialogUpdate};

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

    #[cfg(test)]
    pub(crate) fn get_label(&self) -> &Label {
        &self.label
    }

    pub(crate) fn get_old_descriptor(&self) -> &Descriptor {
        &self.old_descriptor
    }

    #[cfg(test)]
    pub(crate) fn get_new_descriptor(&self) -> &Descriptor {
        &self.new_descriptor
    }
}

impl Dialog for RenameDescriptorDialog {
    fn header(&self) -> String {
        format!(
            "Rename descriptor {} of entity {}",
            self.data.old_descriptor, self.data.label
        )
    }

    fn body(&self) -> Element<'_, GuiMessage> {
        let new_descriptor_input = TextInput::new("", self.data.new_descriptor.to_str())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Descriptor(i.into())));
        let submit_button = Button::new(Text::new("Update")).on_press(GuiMessage::DialogSubmit);
        Column::new()
            .push(Text::new("New Descriptor"))
            .push(new_descriptor_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn update(&mut self, message: super::DialogUpdate) {
        if let DialogUpdate::Descriptor(new_descriptor) = message {
            self.data.new_descriptor = new_descriptor;
        }
    }

    fn submit(&self) -> GuiMessage {
        GuiMessage::RenameDescriptor(self.data.to_owned())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn example_rename_descriptor_data() -> RenameDescriptorData {
        let label = Label::from("example_label");
        let old_descriptor = Descriptor::from("Old Descriptor");
        let new_descriptor = Descriptor::from("New Descriptor");
        RenameDescriptorData {
            label,
            old_descriptor,
            new_descriptor,
        }
    }
}
