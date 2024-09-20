use super::{Dialog, DialogMessage};
use crate::{app::message_handling::GuiMes, errors::LoreGuiError};
use iced::{
    widget::{Button, Column, PickList, Text, TextInput},
    Element,
};
use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{child::Child, parent::Parent, relationship::EntityRelationship, role::Role},
};

#[derive(Debug, Clone)]
pub(crate) struct NewRelationshipDialog {
    parent_labels: Vec<Parent>,
    child_labels: Vec<Child>,
    data: NewRelationshipData,
}

impl NewRelationshipDialog {
    pub(crate) fn new(parent_labels: Vec<Parent>, child_labels: Vec<Child>) -> Self {
        NewRelationshipDialog {
            parent_labels,
            child_labels,
            data: NewRelationshipData::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NewRelationshipData {
    pub(self) parent: Parent,
    pub(self) child: Child,
    pub(self) role: Role,
}

impl NewRelationshipData {
    pub(crate) fn new() -> Self {
        NewRelationshipData {
            parent: String::new().into(),
            child: String::new().into(),
            role: String::new().into(),
        }
    }

    pub(crate) fn write_to_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        if self.parent.to_str().is_empty() || self.child.to_str().is_empty() {
            return Err(LoreGuiError::InputError(
                "Parent and child cannot be empty.".to_string(),
            ));
        }

        let rel = EntityRelationship {
            parent: self.parent,
            child: self.child,
            role: self.role,
        };
        db.write_relationships(vec![rel])
            .map_err(LoreGuiError::from)
    }
}

impl Dialog for NewRelationshipDialog {
    fn header(&self) -> String {
        "New Relationship".to_string()
    }

    fn body<'a>(&'a self) -> Element<'a, GuiMes> {
        let selected_parent = if self.data.parent.to_str().is_empty() {
            None
        } else {
            Some(self.data.parent.clone())
        };
        let parent_input = PickList::new(self.parent_labels.clone(), selected_parent, |s| {
            GuiMes::DialogUpdate(DialogMessage::ParentUpd(s))
        });
        let selected_child = if self.data.child.to_str().is_empty() {
            None
        } else {
            Some(self.data.child.clone())
        };
        let child_input = PickList::new(self.child_labels.clone(), selected_child, |s| {
            GuiMes::DialogUpdate(DialogMessage::ChildUpd(s))
        });
        let role_input = TextInput::new("", self.data.role.to_str())
            .on_input(|i| GuiMes::DialogUpdate(DialogMessage::RoleUpd(i.into())));
        let submit_button = Button::new(Text::new("Create")).on_press(GuiMes::DialogSubmit);
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

    fn update(&mut self, message: DialogMessage) {
        match message {
            DialogMessage::ParentUpd(parent) => {
                self.data.parent = parent;
            }
            DialogMessage::ChildUpd(child) => {
                self.data.child = child;
            }
            DialogMessage::RoleUpd(role) => {
                self.data.role = role;
            }
            _ => (),
        }
    }

    fn submit(&self) -> GuiMes {
        GuiMes::NewRelationship(self.data.to_owned())
    }
}
