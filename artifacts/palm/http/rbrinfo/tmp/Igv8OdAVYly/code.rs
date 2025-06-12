fn from(repr: Repr<T>) -> Bytes {
        match repr {
            Repr::Standard(header) => Bytes::from_static(header.as_str().as_bytes()),
            Repr::Custom(header) => header.into(),
        }
    }