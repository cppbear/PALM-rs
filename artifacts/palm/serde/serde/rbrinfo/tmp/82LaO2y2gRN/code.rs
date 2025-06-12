fn from(self) -> Self::Deserializer {
        BorrowedBytesDeserializer::new(self.0)
    }