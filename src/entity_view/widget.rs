use super::EntityView;
use crate::{
    app::message_handling::GuiMes,
    db_col_view::{widget::DbColView, ColViewMes},
    style::header,
};
use iced::widget::{component, Component};
use iced::{
    widget::{Column, Row, Text},
    Alignment, Element, Length,
};

impl<'a> Component<GuiMes> for EntityView<'a> {
    type State = ();

    type Event = GuiMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        Row::new()
            .push(DbColView::new(
                "Label",
                self.label_button_infos(),
                GuiMes::EntityLabelViewUpd,
                &self.state.label_view_state,
            ))
            .push(DbColView::new(
                "Descriptor",
                self.descriptor_button_infos(),
                GuiMes::DescriptorViewUpd,
                &self.state.descriptor_view_state,
            ))
            .push(self.desription_view())
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<'a> EntityView<'a> {
    fn new_entity_msg(&self) -> Option<ColViewMes> {
        Some(ColViewMes::New)
    }

    fn new_descriptor_msg(&self) -> Option<ColViewMes> {
        if self.state.label_view_state.get_selected().is_some() {
            Some(ColViewMes::New)
        } else {
            None
        }
    }

    fn label_button_infos(&self) -> Vec<(String, Option<ColViewMes>)> {
        vec![
            ("New Entity Label", self.new_entity_msg()),
            ("Delete Entity", None),
            ("Relabel Entity", None),
        ]
        .into_iter()
        .map(|(s, m)| (s.to_string(), m))
        .collect()
    }

    fn descriptor_button_infos(&self) -> Vec<(String, Option<ColViewMes>)> {
        vec![
            ("New Descriptor", self.new_descriptor_msg()),
            ("Delete Descriptor", None),
            ("Rename Descriptor", None),
        ]
        .into_iter()
        .map(|(s, m)| (s.to_string(), m))
        .collect()
    }

    fn desription_view(&self) -> Column<'_, GuiMes> {
        Column::new()
            .push(header("Description"))
            .push(Text::new(match &self.state.current_description {
                Some(description) => description,
                None => "",
            }))
            .padding(5)
            .spacing(5)
            .width(Length::Fill)
    }
}

impl<'a> From<EntityView<'a>> for Element<'a, GuiMes> {
    fn from(entity_view: EntityView<'a>) -> Self {
        component(entity_view)
    }
}
