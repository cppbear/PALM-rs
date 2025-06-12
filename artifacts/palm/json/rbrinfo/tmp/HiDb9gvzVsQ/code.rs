pub fn with_formatter(writer: W, formatter: F) -> Self {
        Serializer { writer, formatter }
    }