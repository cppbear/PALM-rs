fn from(slice: &Slice<T>) -> Self {
        Slice::from_boxed(Box::from(&slice.entries))
    }