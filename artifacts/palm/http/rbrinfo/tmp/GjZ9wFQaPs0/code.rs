pub fn new() -> ByteStr {
        ByteStr {
            // Invariant: the empty slice is trivially valid UTF-8.
            bytes: Bytes::new(),
        }
    }