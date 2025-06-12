pub(super) fn new(entries: &'a [Bucket<T>]) -> Self {
        Self {
            iter: entries.iter(),
        }
    }