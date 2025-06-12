fn eq(&self, other: &BytesMut) -> bool {
        other[..] == self[..]
    }