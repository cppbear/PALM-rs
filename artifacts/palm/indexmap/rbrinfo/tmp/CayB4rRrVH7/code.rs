pub fn into_boxed_slice(self) -> Box<Slice<T>> {
        Slice::from_boxed(self.into_entries().into_boxed_slice())
    }