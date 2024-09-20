use iced::Font;
use iced::{
    widget::{Column, Text, TextInput},
    Element, Length,
};
use iced_aw::{style, SelectionList};
use std::fmt::Display;
use std::hash::Hash;

use super::{state::DbColViewState, ColViewMes};
use crate::{app::message_handling::GuiMes, style::header};

pub(crate) fn new<'a, M, E>(
    title: &'static str,
    gui_message: M,
    state: &'a DbColViewState<E>,
) -> Element<'a, GuiMes>
where
    M: 'static + Clone + Fn(ColViewMes<E>) -> GuiMes,
    E: 'static + Clone + Display + Eq + Hash,
{
    let title = header(title);

    let selected_text = Text::new(format!("Selected: {}", state.get_selected()));

    let search_text = state.get_search_text().unwrap_or("");
    let m = gui_message.clone();
    let search_field_updated = move |s| m(ColViewMes::SearchFieldUpd(s));
    let search_field = TextInput::new("Type to search...", search_text)
        .on_input(search_field_updated)
        .width(Length::Fill);

    let m = gui_message.clone();
    let selection_list_updated = move |i, e| m(ColViewMes::Selected(i, e));
    let selection_list = SelectionList::new_with(
        state.get_entries(),
        selection_list_updated,
        20.0,
        0.0,
        style::selection_list::primary,
        None,
        Font::default(),
    );

    Column::new()
        .push(title)
        .push(selected_text)
        .push(search_field)
        .push(selection_list)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .spacing(5)
        .into()
}
