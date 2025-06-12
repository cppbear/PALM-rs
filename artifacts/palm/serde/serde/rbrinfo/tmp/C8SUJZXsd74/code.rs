fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("U32Deserializer")
            .field("value", &self.value)
            .finish()
    }