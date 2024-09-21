use super::{HistoryViewMessage, HistoryViewState};
use crate::db_col_view;
use crate::dialog::redate_history::RedateHistoryData;
use crate::{app::message_handling::GuiMessage, style::header};
use iced::widget::{button, text_editor};
use iced::Alignment;
use iced::{
    widget::{Column, Row},
    Element, Length,
};

pub(crate) fn new(state: &HistoryViewState) -> Element<'_, GuiMessage> {
    Column::new()
        .push(buttons(state))
        .push(col_views(state))
        .into()
}

fn buttons(state: &HistoryViewState) -> Row<'_, GuiMessage> {
    let new_item = button("New History Item").on_press(GuiMessage::HistoryViewUpd(
        HistoryViewMessage::NewHistoryItem,
    ));
    let mut redate_history = button("Redate History Item");
    let mut delete_item = button("Delete History Item");
    if let (Some(timestamp), Some(year), day) = (
        state.timestamp_view_state.get_selected().0,
        state.year_view_state.get_selected().0,
        state.day_view_state.get_selected(),
    ) {
        let day = day.clone().flatten();
        let redate_history_data = RedateHistoryData::new(timestamp, year, day);
        redate_history = redate_history.on_press(GuiMessage::HistoryViewUpd(
            HistoryViewMessage::RedateHistoryItem(redate_history_data),
        ));
        delete_item = delete_item.on_press(GuiMessage::HistoryViewUpd(
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

fn col_views(state: &HistoryViewState) -> Row<'_, GuiMessage> {
    Row::new()
        .push(db_col_view::widget::new(
            "Year",
            |m| GuiMessage::HistoryViewUpd(HistoryViewMessage::YearViewUpdate(m)),
            &state.year_view_state,
        ))
        .push(db_col_view::widget::new(
            "Day",
            |m| GuiMessage::HistoryViewUpd(HistoryViewMessage::DayViewUpdate(m)),
            &state.day_view_state,
        ))
        .push(db_col_view::widget::new(
            "Timestamp",
            |m| GuiMessage::HistoryViewUpd(HistoryViewMessage::HistoryTimestampViewUpdate(m)),
            &state.timestamp_view_state,
        ))
        .push(content_view(state))
        .align_y(Alignment::Start)
        .width(Length::Fill)
        .height(Length::Fill)
}

fn content_view(state: &HistoryViewState) -> Column<'_, GuiMessage> {
    let content_editor = text_editor(&state.current_content)
        .on_action(|a| GuiMessage::HistoryViewUpd(HistoryViewMessage::ContentUpdate(a)));
    Column::new()
        .push(header("Content"))
        .push(content_editor)
        .padding(5)
        .spacing(5)
        .width(Length::Fill)
}
