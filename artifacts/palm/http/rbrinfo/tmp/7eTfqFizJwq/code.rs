fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            next: None,
            entries: self.entries.into_iter(),
            extra_values: self.extra_values,
        }
    }