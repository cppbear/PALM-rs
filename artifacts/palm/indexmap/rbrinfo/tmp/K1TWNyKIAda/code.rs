fn from(slice: &Slice<K, V>) -> Self {
        Slice::from_boxed(Box::from(&slice.entries))
    }