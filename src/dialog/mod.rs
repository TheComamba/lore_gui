use super::app::message_handling::GuiMessage;
use iced::{
    widget::{Container, Scrollable, Text},
    Element,
};
use iced_aw::{style, Card};
use lorecore::{errors::LoreCoreError, types::*};

pub(crate) mod change_role;
pub(crate) mod confirmation;
pub(crate) mod error;
pub(crate) mod new_descriptor;
pub(crate) mod new_entity;
pub(crate) mod new_history_item;
pub(crate) mod new_relationship;
pub(crate) mod redate_history;
pub(crate) mod relabel_entity;
pub(crate) mod rename_descriptor;

pub(crate) enum CardStyle {
    Primary,
    Warning,
    Error,
}

pub(crate) trait Dialog {
    fn card_style(&self) -> CardStyle {
        CardStyle::Primary
    }

    fn header(&self) -> String;

    fn body(&self) -> Element<'_, GuiMessage>;

    fn update(&mut self, message: DialogUpdate);

    fn submit(&self) -> GuiMessage;

    fn to_element<'a>(&'a self) -> Element<'a, GuiMessage> {
        let header: Text<'a> = Text::new(self.header());
        let body = self.body();
        let card =
            Card::new::<Element<'a, GuiMessage>, Element<'a, GuiMessage>>(header.into(), body)
                .on_close(GuiMessage::DialogClosed);
        let card = match self.card_style() {
            CardStyle::Primary => card.style(style::card::primary),
            CardStyle::Warning => card.style(style::card::warning),
            CardStyle::Error => card.style(style::card::danger),
        };
        Container::new(Scrollable::new(card)).padding(100).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum DialogUpdate {
    Category(String),
    Child(Child),
    Content(HistoryItemContent),
    Day(Result<Day, LoreCoreError>),
    Description(Description),
    Descriptor(Descriptor),
    Label(Label),
    Name(String),
    Parent(Parent),
    Role(Role),
    Year(Result<Year, LoreCoreError>),
}
