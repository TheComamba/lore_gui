use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Day(pub(crate) Option<i32>);

impl Day {
    pub(crate) const NONE: Self = Day(None);
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => value.fmt(f),
            None => "".fmt(f),
        }
    }
}
//TODO: Move file to lorcore?
//TODO: Implenet setter/getter logic like mapping 0 -> None?
//TODO: Change type to u32?
