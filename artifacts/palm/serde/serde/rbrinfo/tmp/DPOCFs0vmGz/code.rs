fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("BorrowedBytesDeserializer")
            .field("value", &self.value)
            .finish()
    }