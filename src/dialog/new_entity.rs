use super::Dialog;
use crate::app::message_handling::GuiMes;
use crate::errors::LoreGuiError;
use iced::widget::{component, Component};
use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::sql::entity::EntityColumn;
use lorecore::sql::lore_database::LoreDatabase;

#[derive(Debug, Clone)]
pub(crate) struct NewEntityDialog {
    data: NewEntityData,
}

impl NewEntityDialog {
    pub(crate) fn new() -> Self {
        NewEntityDialog {
            data: NewEntityData::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NewEntityData {
    pub(self) label: String,
    pub(self) name: String,
    pub(self) category: String,
}

impl NewEntityData {
    pub(crate) fn new() -> Self {
        NewEntityData {
            label: String::new(),
            name: String::new(),
            category: String::new(),
        }
    }

    pub(crate) fn write_to_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        if self.label.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot create entity with empty label.".to_string(),
            ));
        }
        if self.name.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot create entity with empty name.".to_string(),
            ));
        }
        if self.category.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot create entity with empty category.".to_string(),
            ));
        }

        //TODO: These should be constants somewhere.
        let category_descriptor = "_category".to_string();
        let name_descriptor = "_name".to_string();

        let name_col = EntityColumn {
            label: self.label.clone(),
            descriptor: name_descriptor,
            description: Some(self.name),
        };
        let category_col = EntityColumn {
            label: self.label,
            descriptor: category_descriptor,
            description: Some(self.category),
        };

        db.write_entity_columns(vec![name_col])?;
        db.write_entity_columns(vec![category_col])?;

        Ok(())
    }

    pub(crate) fn get_label(&self) -> &str {
        &self.label
    }
}

impl Dialog for NewEntityDialog {
    fn header(&self) -> String {
        "Create new entity".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }
}

impl Component<GuiMes> for NewEntityDialog {
    type State = ();

    type Event = NewEntityMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            NewEntityMes::LabelUpd(label) => {
                self.data.label = label;
                None
            }
            NewEntityMes::CategoryUpd(ent_type) => {
                self.data.category = ent_type;
                None
            }
            NewEntityMes::NameUpd(name) => {
                self.data.name = name;
                None
            }
            NewEntityMes::Submit => Some(GuiMes::NewEntity(self.data.to_owned())),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let label_input = TextInput::new("", &self.data.label).on_input(NewEntityMes::LabelUpd);
        let name_input = TextInput::new("", &self.data.name).on_input(NewEntityMes::NameUpd);
        let category_input =
            TextInput::new("", &self.data.category).on_input(NewEntityMes::CategoryUpd);
        let submit_button = Button::new(Text::new("Create")).on_press(NewEntityMes::Submit);
        Column::new()
            .push(Text::new("Label:"))
            .push(label_input)
            .push(Text::new("Name"))
            .push(name_input)
            .push(Text::new("Category:"))
            .push(category_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewEntityMes {
    LabelUpd(String),
    CategoryUpd(String),
    NameUpd(String),
    Submit,
}
