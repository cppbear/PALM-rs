pub fn from_static(src: &'static str) -> Self {
        let src = Bytes::from_static(src.as_bytes());

        PathAndQuery::from_shared(src).unwrap()
    }