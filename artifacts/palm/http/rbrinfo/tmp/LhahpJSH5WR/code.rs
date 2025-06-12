pub fn keys(&self) -> Keys<'_, T> {
        Keys {
            inner: self.entries.iter(),
        }
    }