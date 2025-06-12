pub(crate) fn into_entries(self: Box<Self>) -> Vec<Bucket<K, V>> {
        self.into_boxed().into_vec()
    }