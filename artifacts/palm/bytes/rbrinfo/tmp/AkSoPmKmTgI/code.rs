fn cmp(&self, other: &BytesMut) -> cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }