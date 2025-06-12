pub(super) fn new(entries: &'a mut [Bucket<K, V>]) -> Self {
        Self {
            iter: entries.iter_mut(),
        }
    }