use std::path::PathBuf;

use super::{message_handling::GuiMessage, SqlGui, ViewType};
use crate::{
    dialog::error::ErrorDialog,
    entity_view::{self, EntityViewState},
    errors::LoreGuiError,
    history_view::{self, HistoryViewState},
    relationship_view::{self, RelationshipViewState},
    user_preferences::load_database_path,
};
use iced::{
    widget::{button, opaque, stack, Button, Column, Container, Row, Text},
    Alignment, Element, Length,
};

impl SqlGui {
    fn new() -> Self {
        let mut gui = SqlGui {
            selected_view: super::ViewType::default(),
            entity_view_state: EntityViewState::default(),
            history_view_state: HistoryViewState::default(),
            relationship_view_state: RelationshipViewState::default(),
            lore_database: None,
            dialog: None,
        };
        if let Some(path) = load_database_path() {
            match gui.initialise(path) {
                Ok(_) => (),
                Err(e) => gui.dialog = Some(Box::new(ErrorDialog::new(e))),
            };
        }
        gui
    }

    pub(crate) fn update(&mut self, message: GuiMessage) {
        if let Err(e) = self.handle_message(message) {
            self.dialog = Some(Box::new(ErrorDialog::new(e)));
        }
    }

    pub(crate) fn view(&self) -> iced::Element<'_, GuiMessage> {
        if let Some(dialog) = self.dialog.as_ref() {
            stack![self.main_view(), opaque(dialog.to_element())].into()
        } else {
            self.main_view()
        }
    }

    fn main_view(&self) -> Element<'_, GuiMessage> {
        let mut col = Column::new()
            .push(self.menu_bar())
            .push(self.current_database_display());
        if self.lore_database.is_some() {
            col = col.push(self.view_selection_bar());
            match self.selected_view {
                ViewType::Entity => {
                    col = col.push(entity_view::widget::new(&self.entity_view_state))
                }
                ViewType::History => {
                    col = col.push(history_view::widget::new(&self.history_view_state))
                }
                ViewType::Relationship => {
                    col = col.push(relationship_view::widget::new(
                        &self.relationship_view_state,
                    ))
                }
            }
        }
        col.height(Length::Fill).into()
    }

    fn menu_bar(&self) -> Element<'_, GuiMessage> {
        Row::new()
            .push(Button::new("New Lore Database").on_press(GuiMessage::NewDatabase))
            .push(Button::new("Open Lore Database").on_press(GuiMessage::OpenDatabase))
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn current_database_display(&self) -> Element<'_, GuiMessage> {
        let content = match self.lore_database.as_ref() {
            Some(db) => db.path_as_string(),
            None => "[No database loaded]".to_string(),
        };
        Container::new(Text::new(content)).padding(5).into()
    }

    fn view_selection_bar(&self) -> Element<'_, GuiMessage> {
        let entity_button =
            button(Text::new("Entities")).on_press(GuiMessage::ViewSelected(ViewType::Entity));
        let history_items_button = button(Text::new("History Items"))
            .on_press(GuiMessage::ViewSelected(ViewType::History));
        let relationships_button = button(Text::new("Relationships"))
            .on_press(GuiMessage::ViewSelected(ViewType::Relationship));
        Row::new()
            .push(entity_button)
            .push(history_items_button)
            .push(relationships_button)
            .width(Length::Fill)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn initialise(&mut self, path: PathBuf) -> Result<(), LoreGuiError> {
        self.open_database(path)?;
        self.update_database_derived_data()?;
        Ok(())
    }
}

impl Default for SqlGui {
    fn default() -> Self {
        SqlGui::new()
    }
}
