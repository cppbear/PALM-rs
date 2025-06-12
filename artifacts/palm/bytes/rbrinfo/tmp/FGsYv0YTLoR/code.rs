fn from(src: &'a str) -> BytesMut {
        BytesMut::from(src.as_bytes())
    }