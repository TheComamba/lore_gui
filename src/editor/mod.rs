use iced::widget::text_editor;

pub(crate) mod widget;

pub(super) struct EditorState {
    pub(super) current_content: text_editor::Content,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            current_content: text_editor::Content::with_text(""),
        }
    }
}

impl EditorState {
    pub(super) fn new(text: &str) -> Self {
        Self {
            current_content: text_editor::Content::with_text(text),
        }
    }

    pub(super) fn perform(&mut self, action: text_editor::Action) {
        self.current_content.perform(action);
    }
}
