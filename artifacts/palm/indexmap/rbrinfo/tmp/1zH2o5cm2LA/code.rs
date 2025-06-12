pub(super) fn new(entries: Vec<Bucket<K, V>>) -> Self {
        Self {
            iter: entries.into_iter(),
        }
    }