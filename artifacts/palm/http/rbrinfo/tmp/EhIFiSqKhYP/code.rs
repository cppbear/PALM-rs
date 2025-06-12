pub fn with_capacity(capacity: usize) -> HeaderMap<T> {
        Self::try_with_capacity(capacity).expect("size overflows MAX_SIZE")
    }