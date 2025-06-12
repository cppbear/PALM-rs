fn from_iter<T: IntoIterator<Item = &'a u8>>(into_iter: T) -> Self {
        BytesMut::from_iter(into_iter.into_iter().copied())
    }