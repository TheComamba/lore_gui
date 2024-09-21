use iced::widget::text_editor;

pub(crate) mod widget;

pub(super) struct EditorState {
    current_content: text_editor::Content,
    persisted_text: String,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            current_content: text_editor::Content::with_text(""),
            persisted_text: String::new(),
        }
    }
}

impl EditorState {
    pub(super) fn new(text: &str) -> Self {
        Self {
            current_content: text_editor::Content::with_text(text),
            persisted_text: text.to_string(),
        }
    }

    pub(super) fn perform(&mut self, action: text_editor::Action) {
        self.current_content.perform(action);
    }

    pub(super) fn get_text(&self) -> String {
        self.current_content.text()
    }

    pub(super) fn reset(&mut self) {
        self.current_content = text_editor::Content::with_text(&self.persisted_text);
    }

    pub(super) fn is_changed(&self) -> bool {
        self.current_content.text() != self.persisted_text
    }

    pub(super) fn save(&mut self) {
        self.persisted_text = self.current_content.text();
    }
}
