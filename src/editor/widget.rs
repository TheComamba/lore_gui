use iced::{
    widget::{button, text_editor, Column},
    Length,
};

use crate::{app::message_handling::GuiMessage, style::header};

use super::EditorState;

pub(crate) fn view<'a, M>(
    title: &'static str,
    state: &'a EditorState,
    on_action: M,
    on_discard: GuiMessage,
    on_save: GuiMessage,
) -> Column<'a, GuiMessage>
where
    M: 'static + Clone + Fn(text_editor::Action) -> GuiMessage,
{
    let editor = text_editor(&state.current_content).on_action(on_action);
    let discard_button = button("Discard").on_press(on_discard);
    let save_button = button("Save").on_press(on_save);

    Column::new()
        .push(header(title))
        .push(editor)
        .push(discard_button)
        .push(save_button)
        .padding(5)
        .spacing(5)
        .width(Length::Fill)
}
