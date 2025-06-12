pub fn remove(self) -> T {
        self.inner.remove_entry().0
    }