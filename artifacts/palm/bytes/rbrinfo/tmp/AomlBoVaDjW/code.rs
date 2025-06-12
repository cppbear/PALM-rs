fn from(slice: &'static [u8]) -> Bytes {
        Bytes::from_static(slice)
    }