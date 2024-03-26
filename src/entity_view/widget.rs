use super::EntityView;
use crate::{
    app::message_handling::GuiMes,
    db_col_view::{widget::DbColView, ColViewMes},
    style::header,
};
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
            .push(self.new_buttons())
            .push(self.col_views())
            .into()
    }
}

impl<'a> EntityView<'a> {
    fn new_buttons(&self) -> Row<'_, GuiMes> {
        let mut row = Row::new()
            .push(button("New Entity Label").on_press(GuiMes::EntityLabelViewUpd(ColViewMes::New)));
        if self.state.label_view_state.get_selected().is_some() {
            row = row.push(
                button("New Descriptor").on_press(GuiMes::DescriptorViewUpd(ColViewMes::New)),
            );
        }
        row.spacing(5).padding(5)
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
            .push(DbColView::new(
                "Label",
                vec![],
                GuiMes::EntityLabelViewUpd,
                &self.state.label_view_state,
            ))
            .push(DbColView::new(
                "Descriptor",
                vec![],
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

impl<'a> From<EntityView<'a>> for Element<'a, GuiMes> {
    fn from(entity_view: EntityView<'a>) -> Self {
        component(entity_view)
    }
}
