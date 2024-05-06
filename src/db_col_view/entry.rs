use std::fmt::Display;

use crate::history_view::day::Day;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct DbColViewEntry<T>(pub(crate) Option<T>);

impl<E> DbColViewEntry<E> {
    pub(crate) const NONE: Self = DbColViewEntry(None);
}

impl DbColViewEntry<Day> {
    pub(crate) fn flatten(self) -> Day {
        self.0.unwrap_or(Day::NONE)
    }
}

impl<T: Display> Display for DbColViewEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => value.fmt(f),
            None => "[none]".fmt(f),
        }
    }
}
