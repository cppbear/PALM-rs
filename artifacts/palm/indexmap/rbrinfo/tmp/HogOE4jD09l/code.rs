fn clone(&self) -> Self {
        Slice::from_boxed(self.entries.to_vec().into_boxed_slice())
    }