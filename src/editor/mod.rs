use iced::widget::text_editor;

pub(crate) mod widget;

pub(super) struct EditorState {
    current_content: text_editor::Content,
    persisted_text: String,
}

impl Default for EditorState {
    fn default() -> Self {
        let current_content = text_editor::Content::with_text("");
        let persisted_text = current_content.text();
        Self {
            current_content,
            persisted_text,
        }
    }
}

impl EditorState {
    pub(super) fn new(text: &str) -> Self {
        let current_content = text_editor::Content::with_text(text);
        let persisted_text = current_content.text();
        Self {
            current_content,
            persisted_text,
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

    pub(super) fn saved(&mut self) {
        self.persisted_text = self.current_content.text();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_state_is_changed() {
        let mut editor_state = EditorState::new(" Test with \\reference {label}.");
        assert_eq!(editor_state.is_changed(), false);
        editor_state.perform(text_editor::Action::Edit(text_editor::Edit::Insert('i')));
        assert_eq!(editor_state.is_changed(), true);
        editor_state.reset();
        assert_eq!(editor_state.is_changed(), false);
    }
}
