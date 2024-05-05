use self::entry::DbColViewEntry;

pub(super) mod entry;
pub(super) mod state;
pub(super) mod widget;

#[derive(Debug, Clone)]
pub(crate) enum ColViewMes<E> {
    SearchFieldUpd(String),
    Selected(usize, DbColViewEntry<E>),
}
