use iced::widget::button;
use iced::Alignment;
use iced::{
    widget::{Column, Row},
    Element, Length,
};
use lorecore::types::relationship::EntityRelationship;

use crate::app::message_handling::GuiMes;
use crate::db_col_view;
use crate::dialog::change_role::ChangeRoleData;

use super::{RelationshipViewMessage, RelationshipViewState};

pub(crate) fn new(state: &RelationshipViewState) -> Element<'_, GuiMes> {
    Column::new()
        .push(buttons(state))
        .push(col_views(state))
        .into()
}

fn buttons(state: &RelationshipViewState) -> Row<'_, GuiMes> {
    let new_relationship = button("New Relationship").on_press(GuiMes::RelationshipViewUpd(
        RelationshipViewMessage::NewRelationship,
    ));
    let mut change_role = button("Change Role");
    let mut delete_relationship = button("Delete Relationship");
    if let (Some(parent), Some(child)) = (
        &state.parent_view_state.get_selected().0,
        &state.child_view_state.get_selected().0,
    ) {
        let role = state
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

fn col_views(state: &RelationshipViewState) -> Row<'_, GuiMes> {
    Row::new()
        .push(db_col_view::widget::new(
            "Parent",
            |m| GuiMes::RelationshipViewUpd(RelationshipViewMessage::ParentViewUpd(m)),
            &state.parent_view_state,
        ))
        .push(db_col_view::widget::new(
            "Child",
            |m| GuiMes::RelationshipViewUpd(RelationshipViewMessage::ChildViewUpd(m)),
            &state.child_view_state,
        ))
        .push(db_col_view::widget::new(
            "Role",
            |m| GuiMes::RelationshipViewUpd(RelationshipViewMessage::RoleViewUpd(m)),
            &state.role_view_state,
        ))
        .align_y(Alignment::Start)
        .width(Length::Fill)
        .height(Length::Fill)
}
