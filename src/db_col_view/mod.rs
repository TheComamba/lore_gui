use self::entry::DbColViewEntry;

pub(super) mod entry;
pub(super) mod state;
pub(super) mod widget;

#[derive(Debug, Clone)]
pub(crate) enum ColViewMes<E: DbColViewEntry> {
    SearchFieldUpd(String),
    Selected(usize, Option<E>),
}
