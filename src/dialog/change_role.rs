use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{relationship::EntityRelationship, role::Role},
};

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::{Dialog, DialogMessage};

#[derive(Debug, Clone)]
pub(crate) struct ChangeRoleDialog {
    data: ChangeRoleData,
}

impl ChangeRoleDialog {
    pub(crate) fn new(data: ChangeRoleData) -> Self {
        ChangeRoleDialog { data }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ChangeRoleData {
    pub(self) old_relationship: EntityRelationship,
    pub(self) new_role: Role,
}

impl ChangeRoleData {
    pub(crate) fn new(old_relationship: EntityRelationship) -> Self {
        ChangeRoleData {
            new_role: old_relationship.role.clone(),
            old_relationship,
        }
    }

    pub(crate) fn write_to_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        db.change_relationship_role(self.old_relationship, &self.new_role)?;
        Ok(())
    }
}

impl Dialog for ChangeRoleDialog {
    fn header(&self) -> String {
        format!(
            "Change role {} for relationship between {} and {}",
            self.data.old_relationship.role.to_str(),
            self.data.old_relationship.parent,
            self.data.old_relationship.child
        )
    }

    fn update(&mut self, event: DialogMessage) {
        match event {
            DialogMessage::NewRoleUpd(new_role) => {
                self.data.new_role = new_role;
            }
            _ => (),
        }
    }

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        let new_role_str = self.data.new_role.to_str();
        let new_role_input = TextInput::new("", new_role_str)
            .on_input(|i| GuiMes::DialogUpdate(DialogMessage::NewRoleUpd(i.into())));
        let submit_button = Button::new(Text::new("Update")).on_press(GuiMes::DialogSubmit);
        Column::new()
            .push(Text::new("New Role"))
            .push(new_role_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn submit(&self) -> GuiMes {
        GuiMes::ChangeRole(self.data.to_owned())
    }
}
