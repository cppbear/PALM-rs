pub fn new(writer: W) -> Self {
        Serializer::with_formatter(writer, CompactFormatter)
    }