fn from(Custom(inner): Custom) -> Bytes {
        Bytes::from(inner)
    }