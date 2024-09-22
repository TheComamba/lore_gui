use iced::widget::button;
use iced::{
    widget::{Column, Row},
    Alignment, Element, Length,
};

use crate::app::message_handling::GuiMessage;
use crate::dialog::relabel_entity::RelabelEntityData;
use crate::dialog::rename_descriptor::RenameDescriptorData;
use crate::{db_col_view, editor};

use super::{EntityViewMessage, EntityViewState};

pub(crate) fn new(state: &EntityViewState) -> Element<'_, GuiMessage> {
    Column::new()
        .push(label_buttons(state))
        .push(descriptor_buttons(state))
        .push(col_views(state))
        .into()
}

fn label_buttons(state: &EntityViewState) -> Row<'_, GuiMessage> {
    let new_entity =
        button("New Entity").on_press(GuiMessage::EntityViewUpd(EntityViewMessage::NewEntity));
    let mut relabel_entity = button("Relabel Entity");
    let mut delete_entity = button("Delete Entity");
    if let Some(label) = state.selected_label() {
        let relabel_entity_data = RelabelEntityData::new(label.clone());
        relabel_entity = relabel_entity.on_press(GuiMessage::EntityViewUpd(
            EntityViewMessage::RelabelEntity(relabel_entity_data),
        ));
        delete_entity = delete_entity.on_press(GuiMessage::EntityViewUpd(
            EntityViewMessage::DeleteEntity(label.clone()),
        ));
    }
    Row::new()
        .push(new_entity)
        .push(relabel_entity)
        .push(delete_entity)
        .spacing(5)
        .padding(5)
}

fn descriptor_buttons(state: &EntityViewState) -> Row<'_, GuiMessage> {
    let mut new_descriptor = button("New Descriptor");
    let mut rename_descriptor = button("Rename Descriptor");
    let mut delete_descriptor = button("Delete Descriptor");
    if let Some(label) = state.selected_label() {
        new_descriptor = new_descriptor.on_press(GuiMessage::EntityViewUpd(
            EntityViewMessage::NewDescriptor(label.clone()),
        ));
        if let Some(descriptor) = state.selected_descriptor() {
            let rename_descriptor_data =
                RenameDescriptorData::new(label.clone(), descriptor.clone());
            rename_descriptor = rename_descriptor.on_press(GuiMessage::EntityViewUpd(
                EntityViewMessage::RenameDescriptor(rename_descriptor_data),
            ));
            delete_descriptor = delete_descriptor.on_press(GuiMessage::EntityViewUpd(
                EntityViewMessage::DeleteDescriptor(label.clone(), descriptor.clone()),
            ));
        }
    }
    Row::new()
        .push(new_descriptor)
        .push(rename_descriptor)
        .push(delete_descriptor)
        .spacing(5)
        .padding(5)
}

fn col_views(state: &EntityViewState) -> Row<'_, GuiMessage> {
    Row::new()
        .push(db_col_view::widget::new(
            "Label",
            |m| GuiMessage::EntityViewUpd(EntityViewMessage::LabelViewUpdate(m)),
            &state.label_view_state,
        ))
        .push(db_col_view::widget::new(
            "Descriptor",
            |m| GuiMessage::EntityViewUpd(EntityViewMessage::DescriptorViewUpdate(m)),
            &state.descriptor_view_state,
        ))
        .push(editor::widget::view(
            "Description",
            &state.current_description,
            |a| GuiMessage::EntityViewUpd(EntityViewMessage::DescriptionUpdate(a)),
            GuiMessage::EntityViewUpd(EntityViewMessage::DescriptionDiscard),
            GuiMessage::EntityViewUpd(EntityViewMessage::DescriptionSave),
        ))
        .align_y(Alignment::Start)
        .width(Length::Fill)
        .height(Length::Fill)
}
