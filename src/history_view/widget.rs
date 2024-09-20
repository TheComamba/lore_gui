use super::{HistoryView, HistoryViewMessage, HistoryViewState};
use crate::db_col_view;
use crate::dialog::redate_history::RedateHistoryData;
use crate::{app::message_handling::GuiMes, style::header};
use iced::widget::{button, text_editor};
use iced::Alignment;
use iced::{
    widget::{Column, Row},
    Element, Length,
};

impl<'a> HistoryView<'a> {
    pub(crate) fn new(state: &'a HistoryViewState) -> Element<'_, GuiMes> {
        Column::new()
            .push(Self::buttons(state))
            .push(Self::col_views(state))
            .into()
    }

    fn buttons(state: &'a HistoryViewState) -> Row<'_, GuiMes> {
        let new_item = button("New History Item")
            .on_press(GuiMes::HistoryViewUpd(HistoryViewMessage::NewHistoryItem));
        let mut redate_history = button("Redate History Item");
        let mut delete_item = button("Delete History Item");
        if let (Some(timestamp), Some(year), day) = (
            state.timestamp_view_state.get_selected().0,
            state.year_view_state.get_selected().0,
            state.day_view_state.get_selected(),
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

    fn col_views(state: &'a HistoryViewState) -> Row<'_, GuiMes> {
        Row::new()
            .push(db_col_view::widget::new(
                "Year",
                |m| GuiMes::HistoryViewUpd(HistoryViewMessage::YearViewUpd(m)),
                &state.year_view_state,
            ))
            .push(db_col_view::widget::new(
                "Day",
                |m| GuiMes::HistoryViewUpd(HistoryViewMessage::DayViewUpd(m)),
                &state.day_view_state,
            ))
            .push(db_col_view::widget::new(
                "Timestamp",
                |m| GuiMes::HistoryViewUpd(HistoryViewMessage::HistoryTimestampViewUpd(m)),
                &state.timestamp_view_state,
            ))
            .push(Self::content_view(state))
            .align_y(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn content_view(state: &'a HistoryViewState) -> Column<'_, GuiMes> {
        Column::new()
            .push(header("Content"))
            .push(text_editor(&state.current_content))
            .padding(5)
            .spacing(5)
            .width(Length::Fill)
    }
}
