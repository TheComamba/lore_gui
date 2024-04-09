use super::Dialog;
use crate::{app::message_handling::GuiMes, errors::LoreGuiError};
use iced::{
    widget::{component, Button, Column, Component, PickList, Text, TextInput},
    Element,
};
use lorecore::sql::{lore_database::LoreDatabase, relationships::EntityRelationship};

#[derive(Debug, Clone)]
pub(crate) struct NewRelationshipDialog {
    parent_labels: Vec<String>,
    child_labels: Vec<String>,
    data: NewRelationshipData,
}

impl NewRelationshipDialog {
    pub(crate) fn new(parent_labels: Vec<String>, child_labels: Vec<String>) -> Self {
        NewRelationshipDialog {
            parent_labels,
            child_labels,
            data: NewRelationshipData::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NewRelationshipData {
    pub(self) parent: String,
    pub(self) child: String,
    pub(self) role: String,
}

impl NewRelationshipData {
    pub(crate) fn new() -> Self {
        NewRelationshipData {
            parent: String::new(),
            child: String::new(),
            role: String::new(),
        }
    }

    pub(crate) fn write_to_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        if self.parent.is_empty() || self.child.is_empty() {
            return Err(LoreGuiError::InputError(
                "Parent and child cannot be empty.".to_string(),
            ));
        }

        let rel = EntityRelationship {
            parent: self.parent,
            child: self.child,
            role: Some(self.role),
        };
        db.write_relationships(vec![rel])
            .map_err(LoreGuiError::from)
    }
}

impl Dialog for NewRelationshipDialog {
    fn header(&self) -> String {
        "New Relationship".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }
}

impl Component<GuiMes> for NewRelationshipDialog {
    type State = ();

    type Event = NewRelationshipMessage;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            NewRelationshipMessage::ParentUpd(parent) => {
                self.data.parent = parent;
                None
            }
            NewRelationshipMessage::ChildUpd(child) => {
                self.data.child = child;
                None
            }
            NewRelationshipMessage::RoleUpd(role) => {
                self.data.role = role;
                None
            }
            NewRelationshipMessage::Submit => Some(GuiMes::NewRelationship(self.data.to_owned())),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let selected_parent = if self.data.parent.is_empty() {
            None
        } else {
            Some(self.data.parent.clone())
        };
        let parent_input = PickList::new(
            self.parent_labels.clone(),
            selected_parent,
            NewRelationshipMessage::ParentUpd,
        );
        let selected_child = if self.data.child.is_empty() {
            None
        } else {
            Some(self.data.child.clone())
        };
        let child_input = PickList::new(
            self.child_labels.clone(),
            selected_child,
            NewRelationshipMessage::ChildUpd,
        );
        let role_input =
            TextInput::new("", &self.data.role).on_input(NewRelationshipMessage::RoleUpd);
        let submit_button =
            Button::new(Text::new("Create")).on_press(NewRelationshipMessage::Submit);
        Column::new()
            .push(Text::new("Parent:"))
            .push(parent_input)
            .push(Text::new("Child:"))
            .push(child_input)
            .push(Text::new("Role:"))
            .push(role_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewRelationshipMessage {
    ParentUpd(String),
    ChildUpd(String),
    RoleUpd(String),
    Submit,
}
