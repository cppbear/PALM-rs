pub(super) fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self {
            iter: entries.iter(),
        }
    }