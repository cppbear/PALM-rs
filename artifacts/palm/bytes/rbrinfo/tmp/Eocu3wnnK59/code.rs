fn from(slice: &'static str) -> Bytes {
        Bytes::from_static(slice.as_bytes())
    }