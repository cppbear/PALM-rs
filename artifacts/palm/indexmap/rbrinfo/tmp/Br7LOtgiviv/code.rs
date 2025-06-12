pub(super) fn new(entries: Vec<Bucket<T>>) -> Self {
        Self {
            iter: entries.into_iter(),
        }
    }