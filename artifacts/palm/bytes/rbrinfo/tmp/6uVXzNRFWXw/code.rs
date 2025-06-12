fn eq(&self, other: &BytesMut) -> bool {
        self.as_slice() == other.as_slice()
    }