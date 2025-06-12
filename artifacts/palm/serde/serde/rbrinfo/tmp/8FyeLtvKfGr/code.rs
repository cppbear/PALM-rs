fn from(self) -> Self::Deserializer {
        BorrowedStrDeserializer {
            value: self.0,
            marker: PhantomData,
        }
    }