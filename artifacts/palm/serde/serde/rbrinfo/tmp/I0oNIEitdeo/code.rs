fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("BytesDeserializer")
            .field("value", &self.value)
            .finish()
    }