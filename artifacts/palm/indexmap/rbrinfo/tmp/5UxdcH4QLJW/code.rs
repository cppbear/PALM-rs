pub(crate) fn into_entries(self: Box<Self>) -> Vec<Bucket<T>> {
        self.into_boxed().into_vec()
    }