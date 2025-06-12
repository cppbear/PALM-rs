pub fn new(value: &'de [u8]) -> Self {
        BorrowedBytesDeserializer {
            value,
            marker: PhantomData,
        }
    }