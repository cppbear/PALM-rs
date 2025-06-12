pub(super) fn star() -> Self {
        PathAndQuery {
            data: ByteStr::from_static("*"),
            query: NONE,
        }
    }