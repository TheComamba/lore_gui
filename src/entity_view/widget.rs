use super::{EntityView, EntityViewMessage};
use crate::db_col_view;
use crate::dialog::relabel_entity::RelabelEntityData;
use crate::dialog::rename_descriptor::RenameDescriptorData;
use crate::{app::message_handling::GuiMes, style::header};
use iced::widget::{button, component, Component};
use iced::{
    widget::{text_editor, Column, Row},
    Alignment, Element, Length,
};

impl<'a> Component<GuiMes> for EntityView<'a> {
    type State = ();

    type Event = GuiMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        Column::new()
            .push(self.label_buttons())
            .push(self.descriptor_buttons())
            .push(self.col_views())
            .into()
    }
}

impl<'a> EntityView<'a> {
    fn label_buttons(&self) -> Row<'_, GuiMes> {
        let new_entity =
            button("New Entity").on_press(GuiMes::EntityViewUpd(EntityViewMessage::NewEntity));
        let mut relabel_entity = button("Relabel Entity");
        let mut delete_entity = button("Delete Entity");
        if let Some(label) = &self.state.label_view_state.get_selected().0 {
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

    fn descriptor_buttons(&self) -> Row<'_, GuiMes> {
        let mut new_descriptor = button("New Descriptor");
        let mut rename_descriptor = button("Rename Descriptor");
        let mut delete_descriptor = button("Delete Descriptor");
        if let Some(label) = &self.state.label_view_state.get_selected().0 {
            new_descriptor = new_descriptor.on_press(GuiMes::EntityViewUpd(
                EntityViewMessage::NewDescriptor(label.clone()),
            ));
            if let Some(descriptor) = &self.state.descriptor_view_state.get_selected().0 {
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

    fn desription_view(&self) -> Column<'_, GuiMes> {
        Column::new()
            .push(header("Description"))
            .push(text_editor(&self.state.current_description))
            .padding(5)
            .spacing(5)
            .width(Length::Fill)
    }

    fn col_views(&self) -> Element<'_, GuiMes> {
        Row::new()
            .push(db_col_view::widget::new(
                "Label",
                |m| GuiMes::EntityViewUpd(EntityViewMessage::LabelViewUpd(m)),
                &self.state.label_view_state,
            ))
            .push(db_col_view::widget::new(
                "Descriptor",
                |m| GuiMes::EntityViewUpd(EntityViewMessage::DescriptorViewUpd(m)),
                &self.state.descriptor_view_state,
            ))
            .push(self.desription_view())
            .align_y(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<'a> From<EntityView<'a>> for Element<'a, GuiMes> {
    fn from(entity_view: EntityView<'a>) -> Self {
        component(entity_view)
    }
}
