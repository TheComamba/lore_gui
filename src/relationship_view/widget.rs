use iced::widget::{button, component, Component};
use iced::Alignment;
use iced::{
    widget::{Column, Row},
    Element, Length,
};
use lorecore::types::relationship::EntityRelationship;

use crate::app::message_handling::GuiMes;
use crate::db_col_view;
use crate::dialog::change_role::ChangeRoleData;

use super::{RelationshipView, RelationshipViewMessage};

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
        let new_relationship = button("New Relationship").on_press(GuiMes::RelationshipViewUpd(
            RelationshipViewMessage::NewRelationship,
        ));
        let mut change_role = button("Change Role");
        let mut delete_relationship = button("Delete Relationship");
        if let (Some(parent), Some(child)) = (
            &self.state.parent_view_state.get_selected().0,
            &self.state.child_view_state.get_selected().0,
        ) {
            let role = self
                .state
                .role_view_state
                .get_selected()
                .0
                .clone()
                .unwrap_or("".into());
            let relationship = EntityRelationship {
                parent: parent.clone(),
                child: child.clone(),
                role,
            };
            let change_role_data = ChangeRoleData::new(relationship.clone());
            change_role = change_role.on_press(GuiMes::RelationshipViewUpd(
                RelationshipViewMessage::ChangeRole(change_role_data),
            ));
            delete_relationship = delete_relationship.on_press(GuiMes::RelationshipViewUpd(
                RelationshipViewMessage::DeleteRelationship(relationship),
            ));
        }
        Row::new()
            .push(new_relationship)
            .push(change_role)
            .push(delete_relationship)
            .spacing(5)
            .padding(5)
    }

    fn col_views(
        &self,
    ) -> iced::advanced::graphics::core::Element<'_, GuiMes, iced::Theme, iced::Renderer> {
        Row::new()
            .push(db_col_view::widget::new(
                "Parent",
                |m| GuiMes::RelationshipViewUpd(RelationshipViewMessage::ParentViewUpd(m)),
                &self.state.parent_view_state,
            ))
            .push(db_col_view::widget::new(
                "Child",
                |m| GuiMes::RelationshipViewUpd(RelationshipViewMessage::ChildViewUpd(m)),
                &self.state.child_view_state,
            ))
            .push(db_col_view::widget::new(
                "Role",
                |m| GuiMes::RelationshipViewUpd(RelationshipViewMessage::RoleViewUpd(m)),
                &self.state.role_view_state,
            ))
            .align_y(Alignment::Start)
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
