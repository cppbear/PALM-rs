pub(super) fn slash() -> Self {
        PathAndQuery {
            data: ByteStr::from_static("/"),
            query: NONE,
        }
    }