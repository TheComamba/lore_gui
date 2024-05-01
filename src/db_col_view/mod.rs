pub(super) mod state;
pub(super) mod widget;
pub(super) mod entry;

#[derive(Debug, Clone)]
pub(crate) enum ColViewMes {
    SearchFieldUpd(String),
    Selected(usize, String),
}
