use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct DbColViewEntry<T>(pub Option<T>);

impl<E> DbColViewEntry<E> {
    pub(crate) const NONE: Self = DbColViewEntry(None);
}

impl<T: Display> Display for DbColViewEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => value.fmt(f),
            None => "[none]".fmt(f),
        }
    }
}
