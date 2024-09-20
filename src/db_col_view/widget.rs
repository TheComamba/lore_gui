use iced::Font;
use iced::{
    widget::{Column, Container, Text, TextInput},
    Element, Length,
};
use iced_aw::{style, SelectionList};
use std::fmt::Display;
use std::hash::Hash;

use crate::{app::message_handling::GuiMes, style::header};

use super::entry::DbColViewEntry;
use super::{state::DbColViewState, ColViewMes};

pub(crate) struct DbColView<'a, M, E> {
    title: &'static str,
    gui_message: M,
    state: &'a DbColViewState<E>,
}

impl<'a, M, E> DbColView<'a, M, E>
where
    M: 'static + Clone + Fn(ColViewMes<E>) -> GuiMes,
    E: 'static + Clone + Display + Eq + Hash,
{
    pub(crate) fn new(title: &'static str, gui_message: M, state: &'a DbColViewState<E>) -> Self {
        Self {
            title,
            gui_message,
            state,
        }
    }

    fn title(&self) -> Text {
        header(self.title)
    }

    fn selected(&self) -> Text {
        let content = format!("Selected: {}", self.state.get_selected());
        Text::new(content)
    }

    fn search_field(&self) -> TextInput<GuiMes> {
        let search_text = self.state.get_search_text().unwrap_or("");
        TextInput::new("Type to search...", search_text)
            .on_input(self.search_field_updated())
            .width(Length::Fill)
    }

    fn search_field_updated(&self) -> impl Fn(String) -> GuiMes {
        let m = self.gui_message.clone();
        move |s| m(ColViewMes::SearchFieldUpd(s))
    }

    fn selection_list(&self) -> Element<GuiMes> {
        let selection_list = SelectionList::new_with(
            self.state.get_entries(),
            self.selection_list_updated(),
            20.0,
            0.0,
            style::selection_list::primary,
            None,
            Font::default(),
        );
        Container::new(selection_list).height(Length::Fill).into()
    }

    fn selection_list_updated(&self) -> impl Fn(usize, DbColViewEntry<E>) -> GuiMes {
        let m = self.gui_message.clone();
        move |i, e| m(ColViewMes::Selected(i, e))
    }

    fn view(&self) -> Element<'_, GuiMes> {
        Column::new()
            .push(self.title())
            .push(self.selected())
            .push(self.search_field())
            .push(self.selection_list())
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .spacing(5)
            .into()
    }
}

impl<'a, M, E> From<DbColView<'a, M, E>> for Element<'a, GuiMes>
where
    M: 'static + Clone + Fn(ColViewMes<E>) -> GuiMes,
    E: 'static + Clone + Display + Eq + Hash,
{
    fn from(col_view: DbColView<'a, M, E>) -> Element<'a, GuiMes> {
        col_view.view()
    }
}
