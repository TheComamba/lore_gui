use iced::{
    widget::{component, Button, Column, Component, Text, TextInput},
    Element,
};
use lorecore::sql::lore_database::LoreDatabase;

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::Dialog;

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
    pub(self) label: String,
    pub(self) old_descriptor: String,
    pub(self) new_descriptor: String,
}

impl RenameDescriptorData {
    pub(crate) fn new(label: String, old_descriptor: String) -> Self {
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
        if self.old_descriptor.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot rename descriptor with empty descriptor.".to_string(),
            ));
        }
        if self.new_descriptor.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot rename descriptor with empty new descriptor.".to_string(),
            ));
        }

        db.change_entity_descriptor((&self.label, self.old_descriptor), &self.new_descriptor)?;
        Ok(())
    }

    pub(crate) fn get_descriptor(&self) -> &str {
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

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }
}

impl Component<GuiMes> for RenameDescriptorDialog {
    type State = ();

    type Event = RenameDescriptorMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            RenameDescriptorMes::NewDescriptorUpd(new_descriptor) => {
                self.data.new_descriptor = new_descriptor;
                None
            }
            RenameDescriptorMes::Submit => Some(GuiMes::RenameDescriptor(self.data.to_owned())),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let new_descriptor_input = TextInput::new("", &self.data.new_descriptor)
            .on_input(RenameDescriptorMes::NewDescriptorUpd);
        let submit_button = Button::new(Text::new("Update")).on_press(RenameDescriptorMes::Submit);
        Column::new()
            .push(Text::new("New Descriptor"))
            .push(new_descriptor_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RenameDescriptorMes {
    NewDescriptorUpd(String),
    Submit,
}
