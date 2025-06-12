fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.into_entries())
    }