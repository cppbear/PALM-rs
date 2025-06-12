fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("StrDeserializer")
            .field("value", &self.value)
            .finish()
    }