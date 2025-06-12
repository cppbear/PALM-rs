pub fn new(inner: T) -> IntoIter<T> {
        IntoIter { inner }
    }