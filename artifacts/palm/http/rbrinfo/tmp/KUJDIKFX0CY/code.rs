fn from(src: String) -> ByteStr {
        ByteStr {
            // Invariant: src is a String so contains valid UTF-8.
            bytes: Bytes::from(src),
        }
    }