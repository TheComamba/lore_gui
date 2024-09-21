use iced::{
    widget::{text_editor, Column},
    Length,
};

use crate::{app::message_handling::GuiMessage, style::header};

use super::EditorState;

fn view<'a, M>(title: &'static str, state: &'a EditorState, on_action: M) -> Column<'a, GuiMessage>
where
    M: 'static + Clone + Fn(text_editor::Action) -> GuiMessage,
{
    let editor = text_editor(&state.current_content).on_action(on_action);
    Column::new()
        .push(header(title))
        .push(editor)
        .padding(5)
        .spacing(5)
        .width(Length::Fill)
}
