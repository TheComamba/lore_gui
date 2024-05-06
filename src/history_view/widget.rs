use super::{HistoryView, HistoryViewMessage};
use crate::dialog::redate_history::RedateHistoryData;
use crate::{app::message_handling::GuiMes, db_col_view::widget::DbColView, style::header};
use iced::widget::{button, component, text_editor, Component};
use iced::Alignment;
use iced::{
    widget::{Column, Row},
    Element, Length,
};

impl<'a> Component<GuiMes> for HistoryView<'a> {
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

impl<'a> HistoryView<'a> {
    fn buttons(&self) -> Row<'_, GuiMes> {
        let new_item = button("New History Item")
            .on_press(GuiMes::HistoryViewUpd(HistoryViewMessage::NewHistoryItem));
        let mut redate_history = button("Redate History Item");
        let mut delete_item = button("Delete History Item");
        if let (Some(timestamp), Some(year), day) = (
            self.state.timestamp_view_state.get_selected().0,
            self.state.year_view_state.get_selected().0,
            self.state.day_view_state.get_selected(),
        ) {
            let day = day.clone().flatten();
            let redate_history_data = RedateHistoryData::new(timestamp, year, day);
            redate_history = redate_history.on_press(GuiMes::HistoryViewUpd(
                HistoryViewMessage::RedateHistoryItem(redate_history_data),
            ));
            delete_item = delete_item.on_press(GuiMes::HistoryViewUpd(
                HistoryViewMessage::DeleteHistoryItem(timestamp),
            ));
        }
        Row::new()
            .push(new_item)
            .push(redate_history)
            .push(delete_item)
            .spacing(5)
            .padding(5)
    }

    fn content_view(&self) -> Column<'_, GuiMes> {
        Column::new()
            .push(header("Content"))
            .push(text_editor(&self.state.current_content))
            .padding(5)
            .spacing(5)
            .width(Length::Fill)
    }

    fn col_views(
        &self,
    ) -> iced::advanced::graphics::core::Element<'_, GuiMes, iced::Theme, iced::Renderer> {
        Row::new()
            .push(DbColView::new(
                "Year",
                |m| GuiMes::HistoryViewUpd(HistoryViewMessage::YearViewUpd(m)),
                &self.state.year_view_state,
            ))
            .push(DbColView::new(
                "Day",
                |m| GuiMes::HistoryViewUpd(HistoryViewMessage::DayViewUpd(m)),
                &self.state.day_view_state,
            ))
            .push(DbColView::new(
                "Timestamp",
                |m| GuiMes::HistoryViewUpd(HistoryViewMessage::HistoryTimestampViewUpd(m)),
                &self.state.timestamp_view_state,
            ))
            .push(self.content_view())
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<'a> From<HistoryView<'a>> for Element<'a, GuiMes> {
    fn from(entity_view: HistoryView<'a>) -> Self {
        component(entity_view)
    }
}
