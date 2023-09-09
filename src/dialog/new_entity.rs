use super::Dialog;
use crate::app::message_handling::GuiMes;
use iced::widget::{component, Component};
use iced::{
    widget::{Button, Column, Text, TextInput},
    Element, Renderer,
};

#[derive(Debug, Clone)]
pub(crate) struct NewEntityDialog {
    label: String,
    ent_category: String,
    name: String,
}

impl NewEntityDialog {
    pub(crate) fn new() -> Self {
        NewEntityDialog {
            label: String::new(),
            ent_category: String::new(),
            name: String::new(),
        }
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

impl Component<GuiMes, Renderer> for NewEntityDialog {
    type State = ();

    type Event = NewEntityMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            NewEntityMes::LabelUpd(label) => {
                self.label = label;
                None
            }
            NewEntityMes::CategoryUpd(ent_type) => {
                self.ent_category = ent_type;
                None
            }
            NewEntityMes::NameUpd(name) => {
                self.name = name;
                None
            }
            NewEntityMes::Submit => Some(GuiMes::EntitySubmit),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let label_input = TextInput::new("", &self.label).on_input(NewEntityMes::LabelUpd);
        let type_input = TextInput::new("", &self.ent_category).on_input(NewEntityMes::CategoryUpd);
        let name_input = TextInput::new("", &self.name).on_input(NewEntityMes::NameUpd);
        let submit_button = Button::new(Text::new("Create")).on_press(NewEntityMes::Submit);
        Column::new()
            .push(Text::new("Label:"))
            .push(label_input)
            .push(Text::new("Category:"))
            .push(type_input)
            .push(Text::new("Name"))
            .push(name_input)
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
