fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
        IterMut2::new(self.as_entries_mut())
    }