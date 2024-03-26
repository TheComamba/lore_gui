use super::{RelationshipView, RelationshipViewMessage};
use crate::{app::message_handling::GuiMes, db_col_view::widget::DbColView, style::header};
use iced::widget::{button, component, Component};
use iced::Alignment;
use iced::{
    widget::{Column, Row, Text},
    Element, Length,
};

impl<'a> Component<GuiMes> for RelationshipView<'a> {
    type State = ();

    type Event = GuiMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        Column::new()
            .push(self.buttons())
            .push(self.col_views())
            .into()
    }
}

impl<'a> RelationshipView<'a> {
    fn buttons(&self) -> Row<'_, GuiMes> {
        Row::new()
            .push(
                button("New Relationship").on_press(GuiMes::RelationshipViewUpd(
                    RelationshipViewMessage::NewRelationship,
                )),
            )
            .spacing(5)
            .padding(5)
    }

    fn role_view(&self) -> Element<'a, GuiMes> {
        let mut col = Column::new().push(header("Role"));
        if let Some(role) = self.state.current_role.as_ref() {
            col = col.push(Text::new(role));
        }
        col.padding(5).spacing(5).width(Length::Fill).into()
    }

    fn col_views(
        &self,
    ) -> iced::advanced::graphics::core::Element<'_, GuiMes, iced::Theme, iced::Renderer> {
        Row::new()
            .push(DbColView::new(
                "Parent",
                |m| GuiMes::RelationshipViewUpd(RelationshipViewMessage::ParentViewUpd(m)),
                &self.state.parent_view_state,
            ))
            .push(DbColView::new(
                "Child",
                |m| GuiMes::RelationshipViewUpd(RelationshipViewMessage::ChildViewUpd(m)),
                &self.state.child_view_state,
            ))
            .push(self.role_view())
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<'a> From<RelationshipView<'a>> for Element<'a, GuiMes> {
    fn from(entity_view: RelationshipView<'a>) -> Self {
        component(entity_view)
    }
}
