pub(super) trait DbColViewEntry {
    fn column_representation(&self) -> String;
}

impl DbColViewEntry for String {
    fn column_representation(&self) -> String {
        self.clone()
    }
}

impl DbColViewEntry for i32 {
    fn column_representation(&self) -> String {
        self.to_string()
    }
}

impl DbColViewEntry for i64 {
    fn column_representation(&self) -> String {
        self.to_string()
    }
}

impl<T: DbColViewEntry> DbColViewEntry for Option<T> {
    fn column_representation(&self) -> String {
        match self {
            Some(value) => value.column_representation(),
            None => String::new(),
        }
    }
}
