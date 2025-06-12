fn clone(&self) -> BytesMut {
        BytesMut::from(&self[..])
    }