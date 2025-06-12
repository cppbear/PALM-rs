fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("StringDeserializer")
            .field("value", &self.value)
            .finish()
    }