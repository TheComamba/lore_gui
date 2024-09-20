use super::app::message_handling::GuiMes;
use iced::{
    widget::{Container, Scrollable, Text},
    Element,
};
use iced_aw::{style, Card};
use lorecore::{
    errors::LoreCoreError,
    types::{
        child::Child, day::Day, description::Description, descriptor::Descriptor,
        history_item_content::HistoryItemContent, label::Label, parent::Parent, role::Role,
        year::Year,
    },
};

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

    fn body<'a>(&'a self) -> Element<'a, GuiMes>;

    fn update(&mut self, message: DialogMessage);

    fn submit(&self) -> GuiMes;

    fn to_element<'a>(&'a self) -> Element<'a, GuiMes> {
        let header: Text<'a> = Text::new(self.header());
        let body = self.body();
        let card = Card::new::<Element<'a, GuiMes>, Element<'a, GuiMes>>(header.into(), body)
            .on_close(GuiMes::DialogClosed);
        let card = match self.card_style() {
            CardStyle::Primary => card.style(style::card::primary),
            CardStyle::Warning => card.style(style::card::warning),
            CardStyle::Error => card.style(style::card::danger),
        };
        Container::new(Scrollable::new(card)).padding(100).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum DialogMessage {
    DescriptorUpd(Descriptor),
    DescriptionUpd(Description),
    LabelUpd(Label),
    CategoryUpd(String),
    NameUpd(String),
    YearUpd(Result<Year, LoreCoreError>),
    DayUpd(Result<Day, LoreCoreError>),
    ContentUpd(HistoryItemContent),
    ParentUpd(Parent),
    ChildUpd(Child),
    RoleUpd(Role),
    NewRoleUpd(Role),
    NewLabelUpd(Label),
    NewDescriptorUpd(Descriptor),
}
