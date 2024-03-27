use iced::{
    widget::{component, Button, Column, Component, Text, TextInput},
    Element,
};
use lorecore::sql::lore_database::LoreDatabase;

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
    pub(self) parent: String,
    pub(self) child: String,
    pub(self) old_role: String,
    pub(self) new_role: String,
}

impl ChangeRoleData {
    pub(crate) fn new(parent: String, child: String, old_role: String) -> Self {
        ChangeRoleData {
            parent,
            child,
            new_role: old_role.clone(),
            old_role,
        }
    }

    pub(crate) fn update_role_in_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        if self.old_role.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot change role with empty role.".to_string(),
            ));
        }
        if self.new_role.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot change role with empty new role.".to_string(),
            ));
        }

        todo!();
        //db.update_role_name(&self.role, &self.new_role)?;

        Ok(())
    }

    pub(crate) fn get_role(&self) -> &str {
        &self.old_role
    }
}

impl Dialog for ChangeRoleDialog {
    fn header(&self) -> String {
        format!(
            "Change role {} for relationship between {} and {}",
            self.data.old_role, self.data.parent, self.data.child
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
        let new_role_input =
            TextInput::new("", &self.data.new_role).on_input(ChangeRoleMes::NewRoleUpd);
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
    NewRoleUpd(String),
    Submit,
}
