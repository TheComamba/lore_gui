use iced::widget::text_editor;

pub(crate) mod widget;

pub(super) struct EditorState {
    pub(super) current_content: text_editor::Content,
}

impl EditorState {
    pub(super) fn new() -> Self {
        Self {
            current_content: text_editor::Content::with_text(""),
        }
    }

    pub(super) fn perform(&mut self, action: text_editor::Action) {
        self.current_content.perform(action);
    }
}
