pub fn as_slice(&self) -> &'a Slice<T> {
        Slice::from_slice(self.iter.as_slice())
    }