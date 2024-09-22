use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{sql::lore_database::LoreDatabase, types::label::Label};

use crate::{app::message_handling::GuiMessage, errors::LoreGuiError};

use super::{Dialog, DialogUpdate};

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

    pub(crate) fn new_label(&self) -> &Label {
        &self.new_label
    }

    #[cfg(test)]
    pub(crate) fn old_label(&self) -> &Label {
        &self.old_label
    }
}

impl Dialog for RelabelEntityDialog {
    fn header(&self) -> String {
        format!("Relabel entity: {}", self.data.old_label)
    }

    fn body(&self) -> Element<'_, GuiMessage> {
        let new_label_input = TextInput::new("", self.data.new_label.to_str())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Label(i.into())));
        let submit_button = Button::new(Text::new("Update")).on_press(GuiMessage::DialogSubmit);
        Column::new()
            .push(Text::new("New Label"))
            .push(new_label_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn update(&mut self, message: DialogUpdate) {
        if let DialogUpdate::Label(new_label) = message {
            self.data.new_label = new_label;
        }
    }

    fn submit(&self) -> GuiMessage {
        GuiMessage::RelabelEntity(self.data.to_owned())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use lorecore::types::label::Label;

    use super::RelabelEntityData;

    pub(crate) fn example_relabel_entity_data() -> RelabelEntityData {
        let old_label = Label::from("old_entity");
        let new_label = Label::from("new_entity");
        RelabelEntityData {
            old_label,
            new_label,
        }
    }
}
