use super::app::message_handling::GuiMes;
use iced::{
    widget::{Container, Scrollable, Text},
    Element, Renderer,
};
use iced_aw::{style::CardStyles, Card};

pub(crate) mod error;
pub(crate) mod new_descriptor;
pub(crate) mod new_entity;
pub(crate) mod new_history_item;

pub(crate) trait Dialog {
    fn card_style(&self) -> CardStyles {
        CardStyles::Primary
    }

    fn header(&self) -> String;

    fn body<'a>(&self) -> Element<'a, GuiMes>;

    fn to_element<'a>(&self) -> Element<'a, GuiMes> {
        let header: Text<'a, Renderer> = Text::new(self.header());
        let body = self.body();
        let card = Card::new::<Element<'a, GuiMes>, Element<'a, GuiMes>>(header.into(), body)
            .style(self.card_style())
            .on_close(GuiMes::DialogClosed);
        Container::new(Scrollable::new(card)).padding(100).into()
    }
}
