use iced::{
    widget::{component, Button, Column, Component, Text, TextInput},
    Element,
};
use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{relationship::EntityRelationship, role::Role},
};

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::Dialog;

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

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }
}

impl Component<GuiMes> for ChangeRoleDialog {
    type State = ();

    type Event = ChangeRoleMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            ChangeRoleMes::NewRoleUpd(new_role) => {
                self.data.new_role = new_role;
                None
            }
            ChangeRoleMes::Submit => Some(GuiMes::ChangeRole(self.data.to_owned())),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let new_role_str = self.data.new_role.to_str();
        let new_role_input =
            TextInput::new("", new_role_str).on_input(|i| ChangeRoleMes::NewRoleUpd(i.into()));
        let submit_button = Button::new(Text::new("Update")).on_press(ChangeRoleMes::Submit);
        Column::new()
            .push(Text::new("New Role"))
            .push(new_role_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ChangeRoleMes {
    NewRoleUpd(Role),
    Submit,
}
