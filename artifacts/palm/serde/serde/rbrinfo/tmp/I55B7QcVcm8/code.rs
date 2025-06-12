fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("CowStrDeserializer")
            .field("value", &self.value)
            .finish()
    }