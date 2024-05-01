use std::fmt::Display;
use std::hash::Hash;

use super::entry::DbColViewEntry;
use super::{state::DbColViewState, ColViewMes};
use crate::{app::message_handling::GuiMes, style::header};
use iced::widget::{component, Component};
use iced::Font;
use iced::{
    widget::{Column, Container, Text, TextInput},
    Element, Length,
};
use iced_aw::{style::SelectionListStyles, SelectionList};

pub(crate) struct DbColView<'a, M, E: DbColViewEntry> {
    title: &'a str,
    gui_message: M,
    state: &'a DbColViewState<E>,
}

impl<'a, M, E> DbColView<'a, M, E>
where
    M: 'static + Clone + Fn(ColViewMes<E>) -> GuiMes,
    E: DbColViewEntry + Clone,
{
    pub(crate) fn new(title: &'a str, gui_message: M, state: &'a DbColViewState<E>) -> Self {
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
        let content = "Selected: ".to_string()
            + match self.state.get_selected() {
                Some(sel) => &sel.column_representation(),
                None => "[None]",
            };
        Text::new(content)
    }

    fn search_field(&self) -> TextInput<ColViewMes<E>> {
        let search_text = self.state.get_search_text().unwrap_or("");
        TextInput::new("Type to search...", search_text)
            .on_input(ColViewMes::SearchFieldUpd)
            .width(Length::Fill)
    }

    fn selection_list(&self) -> Element<ColViewMes<E>> {
        let selection_list = SelectionList::new_with(
            self.state.get_entries(),
            ColViewMes::Selected,
            20.0,
            0.0,
            SelectionListStyles::Default,
            None,
            Font::default(),
        );
        Container::new(selection_list).height(Length::Fill).into()
    }
}

impl<'a, M, E: DbColViewEntry> Component<GuiMes> for DbColView<'a, M, E>
where
    M: 'static + Clone + Fn(ColViewMes<E>) -> GuiMes,
{
    type State = DbColViewState<E>;

    type Event = ColViewMes<E>;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        let m = self.gui_message.clone();
        Some(m(event))
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
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

impl<'a, M, E: DbColViewEntry> From<DbColView<'a, M, E>> for Element<'a, GuiMes>
where
    M: 'static + Clone + Fn(ColViewMes<E>) -> GuiMes,
{
    fn from(col_view: DbColView<'a, M, E>) -> Self {
        component(col_view)
    }
}
