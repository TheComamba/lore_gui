use iced::{
    widget::{Button, Column, PickList, Text, TextInput},
    Element,
};
use lorecore::{sql::lore_database::LoreDatabase, types::*};

use crate::{app::message_handling::GuiMessage, errors::LoreGuiError};

use super::{Dialog, DialogUpdate};

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

    #[cfg(test)]
    pub(crate) fn parent(&self) -> &Parent {
        &self.parent
    }

    #[cfg(test)]
    pub(crate) fn child(&self) -> &Child {
        &self.child
    }

    #[cfg(test)]
    pub(crate) fn role(&self) -> &Role {
        &self.role
    }
}

impl Dialog for NewRelationshipDialog {
    fn header(&self) -> String {
        "New Relationship".to_string()
    }

    fn body(&self) -> Element<'_, GuiMessage> {
        let selected_parent = if self.data.parent.to_str().is_empty() {
            None
        } else {
            Some(self.data.parent.clone())
        };
        let parent_input = PickList::new(self.parent_labels.clone(), selected_parent, |s| {
            GuiMessage::DialogUpdate(DialogUpdate::Parent(s))
        });
        let selected_child = if self.data.child.to_str().is_empty() {
            None
        } else {
            Some(self.data.child.clone())
        };
        let child_input = PickList::new(self.child_labels.clone(), selected_child, |s| {
            GuiMessage::DialogUpdate(DialogUpdate::Child(s))
        });
        let role_input = TextInput::new("", self.data.role.to_str())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Role(i.into())));
        let submit_button = Button::new(Text::new("Create")).on_press(GuiMessage::DialogSubmit);
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

    fn update(&mut self, message: DialogUpdate) {
        match message {
            DialogUpdate::Parent(parent) => {
                self.data.parent = parent;
            }
            DialogUpdate::Child(child) => {
                self.data.child = child;
            }
            DialogUpdate::Role(role) => {
                self.data.role = role;
            }
            _ => (),
        }
    }

    fn submit(&self) -> GuiMessage {
        GuiMessage::NewRelationship(self.data.to_owned())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn example_new_relationship_data() -> NewRelationshipData {
        NewRelationshipData {
            parent: "parent".into(),
            child: "child".into(),
            role: "role".into(),
        }
    }
}
