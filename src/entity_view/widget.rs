use super::{EntityViewMessage, EntityViewState};
use crate::db_col_view;
use crate::dialog::relabel_entity::RelabelEntityData;
use crate::dialog::rename_descriptor::RenameDescriptorData;
use crate::{app::message_handling::GuiMes, style::header};
use iced::widget::button;
use iced::{
    widget::{text_editor, Column, Row},
    Alignment, Element, Length,
};

pub(crate) fn new(state: &EntityViewState) -> Element<'_, GuiMes> {
    Column::new()
        .push(label_buttons(state))
        .push(descriptor_buttons(state))
        .push(col_views(state))
        .into()
}

fn label_buttons(state: &EntityViewState) -> Row<'_, GuiMes> {
    let new_entity =
        button("New Entity").on_press(GuiMes::EntityViewUpd(EntityViewMessage::NewEntity));
    let mut relabel_entity = button("Relabel Entity");
    let mut delete_entity = button("Delete Entity");
    if let Some(label) = &state.label_view_state.get_selected().0 {
        let relabel_entity_data = RelabelEntityData::new(label.clone());
        relabel_entity = relabel_entity.on_press(GuiMes::EntityViewUpd(
            EntityViewMessage::RelabelEntity(relabel_entity_data),
        ));
        delete_entity = delete_entity.on_press(GuiMes::EntityViewUpd(
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

fn descriptor_buttons(state: &EntityViewState) -> Row<'_, GuiMes> {
    let mut new_descriptor = button("New Descriptor");
    let mut rename_descriptor = button("Rename Descriptor");
    let mut delete_descriptor = button("Delete Descriptor");
    if let Some(label) = &state.label_view_state.get_selected().0 {
        new_descriptor = new_descriptor.on_press(GuiMes::EntityViewUpd(
            EntityViewMessage::NewDescriptor(label.clone()),
        ));
        if let Some(descriptor) = &state.descriptor_view_state.get_selected().0 {
            let rename_descriptor_data =
                RenameDescriptorData::new(label.clone(), descriptor.clone());
            rename_descriptor = rename_descriptor.on_press(GuiMes::EntityViewUpd(
                EntityViewMessage::RenameDescriptor(rename_descriptor_data),
            ));
            delete_descriptor = delete_descriptor.on_press(GuiMes::EntityViewUpd(
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

fn col_views(state: &EntityViewState) -> Row<'_, GuiMes> {
    Row::new()
        .push(db_col_view::widget::new(
            "Label",
            |m| GuiMes::EntityViewUpd(EntityViewMessage::LabelViewUpd(m)),
            &state.label_view_state,
        ))
        .push(db_col_view::widget::new(
            "Descriptor",
            |m| GuiMes::EntityViewUpd(EntityViewMessage::DescriptorViewUpd(m)),
            &state.descriptor_view_state,
        ))
        .push(desription_view(state))
        .align_y(Alignment::Start)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn desription_view(state: &EntityViewState) -> Column<'_, GuiMes> {
    Column::new()
        .push(header("Description"))
        .push(text_editor(&state.current_description))
        .padding(5)
        .spacing(5)
        .width(Length::Fill)
}
