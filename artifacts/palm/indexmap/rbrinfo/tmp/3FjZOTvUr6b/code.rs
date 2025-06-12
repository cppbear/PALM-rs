pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(self.as_entries())
    }