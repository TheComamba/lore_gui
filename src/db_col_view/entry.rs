use std::fmt::Display;

#[derive(Clone, Debug)]
pub(super) struct DbColViewEntry<T>(pub Option<T>);

impl<T> Display for DbColViewEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Some(value) => value.fmt(f),
            None => "[none]".fmt(f),
        }
    }
}
