use lorecore::{sql::lore_database::LoreDatabase, types::*};

use crate::{
    app::state::GuiState,
    db_col_view::{entry::DbColViewEntry, ColViewMes},
    dialog::{
        confirmation::ConfirmationDialog,
        new_descriptor::{NewDescriptorData, NewDescriptorDialog},
        new_entity::{NewEntityData, NewEntityDialog},
        relabel_entity::{RelabelEntityData, RelabelEntityDialog},
        rename_descriptor::{RenameDescriptorData, RenameDescriptorDialog},
    },
    entity_view::{EntityViewMessage, EntityViewState},
    errors::LoreGuiError,
};

use super::message_handling::GuiMessage;

impl GuiState {
    pub(super) fn update_entity_view(
        &mut self,
        event: EntityViewMessage,
    ) -> Result<(), LoreGuiError> {
        match event {
            EntityViewMessage::NewEntity => self.dialog = Some(Box::new(NewEntityDialog::new())),
            EntityViewMessage::RelabelEntity(data) => {
                self.dialog = Some(Box::new(RelabelEntityDialog::new(data)))
            }
            EntityViewMessage::DeleteEntity(label) => {
                let message = format!("Do you really want to delete {}?", label);
                let on_confirm = GuiMessage::DeleteEntity(label);
                self.dialog = Some(Box::new(ConfirmationDialog::new(message, on_confirm)))
            }
            EntityViewMessage::NewDescriptor(label) => {
                self.dialog = Some(Box::new(NewDescriptorDialog::new(label.clone())))
            }
            EntityViewMessage::RenameDescriptor(data) => {
                self.dialog = Some(Box::new(RenameDescriptorDialog::new(data)))
            }
            EntityViewMessage::DeleteDescriptor(label, descriptor) => {
                let message = format!(
                    "Do you really want to delete {}'s descriptor {}?",
                    label, descriptor
                );
                let on_confirm = GuiMessage::DeleteDescriptor(label, descriptor);
                self.dialog = Some(Box::new(ConfirmationDialog::new(message, on_confirm)))
            }
            EntityViewMessage::LabelViewUpdate(event) => self.update_label_view(event)?,
            EntityViewMessage::DescriptorViewUpdate(event) => self.update_descriptor_view(event)?,
            EntityViewMessage::DescriptionUpdate(action) => {
                self.entity_view_state.current_description.perform(action)
            }
            EntityViewMessage::DescriptionDiscard => {
                self.entity_view_state.current_description.reset()
            }
            EntityViewMessage::DescriptionSave => {
                let db = self
                    .lore_database
                    .as_ref()
                    .ok_or(LoreGuiError::NoDatabase)?;
                let label = match self.get_selected_label() {
                    Some(label) => label,
                    None => return Ok(()),
                };
                let descriptor = match self.get_selected_descriptor() {
                    Some(descriptor) => descriptor,
                    None => return Ok(()),
                };
                let description = self.get_description_text().into();
                db.change_entity_description((&label, &descriptor), &description)?;
                self.entity_view_state.current_description.saved();
            }
        };
        self.entity_view_state.update(&self.lore_database)?;
        Ok(())
    }

    pub(super) fn update_label_view(
        &mut self,
        event: ColViewMes<Label>,
    ) -> Result<(), LoreGuiError> {
        let state = &mut self.entity_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.label_view_state.set_search_text(text);
            }
            ColViewMes::Selected(_index, label) => {
                state.set_selected_label(label.0);
                state.set_selected_descriptor(None);
            }
        };
        self.entity_view_state.update(&self.lore_database)?;
        Ok(())
    }

    pub(super) fn update_descriptor_view(
        &mut self,
        event: ColViewMes<Descriptor>,
    ) -> Result<(), LoreGuiError> {
        let state = &mut self.entity_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.descriptor_view_state.set_search_text(text);
            }
            ColViewMes::Selected(_index, descriptor) => {
                state.set_selected_descriptor(descriptor.0);
            }
        };

        self.entity_view_state.update(&self.lore_database)?;
        Ok(())
    }

    pub(super) fn write_new_entity(&mut self, data: NewEntityData) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let label = data.label().clone();
        data.write_to_database(db)?;
        self.set_selected_label(Some(label));
        Ok(())
    }

    pub(super) fn relabel_entity(&mut self, data: RelabelEntityData) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let new_label = data.new_label().clone();
        data.update_label_in_database(db)?;
        self.set_selected_label(Some(new_label));
        Ok(())
    }

    pub(super) fn delete_entity(&mut self, label: Label) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        db.delete_entity(label)?;
        self.set_selected_label(None);
        Ok(())
    }

    pub(super) fn write_new_descriptor(
        &mut self,
        data: NewDescriptorData,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let label = data.label().clone();
        let descriptor = data.descriptor().clone();
        data.write_to_database(db)?;
        self.set_selected_label(Some(label));
        self.set_selected_descriptor(Some(descriptor));
        Ok(())
    }

    pub(super) fn change_descriptor(
        &mut self,
        data: RenameDescriptorData,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let label = data.label().clone();
        let descriptor = data.new_descriptor().clone();
        data.update_descriptor_in_database(db)?;
        self.set_selected_label(Some(label));
        self.set_selected_descriptor(Some(descriptor));
        Ok(())
    }

    pub(super) fn delete_descriptor(
        &mut self,
        label: Label,
        descriptor: Descriptor,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        db.delete_entity_column((label.clone(), descriptor))?;
        self.set_selected_label(Some(label));
        self.set_selected_descriptor(None);
        Ok(())
    }
}

impl EntityViewState {
    pub(super) fn reset_selections(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<(), LoreGuiError> {
        self.set_selected_label(None);
        self.set_selected_descriptor(None);
        self.set_description_text("");
        self.update(db)?;
        Ok(())
    }

    pub(super) fn update(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        self.update_labels(db)?;
        self.update_descriptors(db)?;
        self.update_description(db)?;
        Ok(())
    }

    fn update_labels(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let labels = self
            .get_current_labels(db)?
            .into_iter()
            .map(|l| DbColViewEntry(Some(l)))
            .collect();
        self.label_view_state.set_entries(labels);
        Ok(())
    }

    fn update_descriptors(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let descriptors = self
            .get_current_descriptors(db)?
            .into_iter()
            .map(|d| DbColViewEntry(Some(d)))
            .collect();
        self.descriptor_view_state.set_entries(descriptors);
        Ok(())
    }

    fn update_description(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let description = self.get_current_description(db)?;
        self.set_description_text(description.to_str());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::{example_database, example_descriptors, example_labels};

    #[test]
    fn selecting_label_deselects_descriptor() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let labels = example_labels();
        let descriptors = example_descriptors();
        gui.set_selected_label(Some(labels[0].clone()));
        gui.set_selected_descriptor(Some(descriptors[0].0.clone()));
        gui.set_description_text(descriptors[0].1.to_str());

        let new_label = labels[1].clone();
        let event = ColViewMes::Selected(1, DbColViewEntry(Some(new_label.clone())));
        gui.update_label_view(event).unwrap();

        assert_eq!(gui.get_selected_label(), Some(new_label),);
        assert_eq!(gui.get_selected_descriptor(), None);
        assert_eq!(gui.get_description_text(), "\n");
    }

    #[test]
    fn selecting_descriptor_updates_description() {
        let mut gui = GuiState {
            lore_database: Some(example_database()),
            ..Default::default()
        };
        let labels = example_labels();
        let descriptors = example_descriptors();
        gui.set_selected_label(Some(labels[0].clone()));
        gui.set_selected_descriptor(Some(descriptors[0].0.clone()));
        gui.set_description_text(descriptors[0].1.to_str());

        let new_descriptor = descriptors[1].0.clone();
        let event = ColViewMes::Selected(1, DbColViewEntry(Some(new_descriptor.clone())));
        gui.update_descriptor_view(event).unwrap();

        assert_eq!(gui.get_selected_label(), Some(labels[0].clone()));
        assert_eq!(gui.get_selected_descriptor(), Some(new_descriptor));
        assert_eq!(gui.get_description_text(), descriptors[1].1.to_str());
    }
}
