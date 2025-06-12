fn from_iter<T: IntoIterator<Item = u8>>(into_iter: T) -> Self {
        Vec::from_iter(into_iter).into()
    }