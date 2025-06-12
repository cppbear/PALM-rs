fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("BorrowedStrDeserializer")
            .field("value", &self.value)
            .finish()
    }