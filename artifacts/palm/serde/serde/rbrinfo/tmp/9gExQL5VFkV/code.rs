pub fn new(value: &'de str) -> BorrowedStrDeserializer<'de, E> {
        BorrowedStrDeserializer {
            value,
            marker: PhantomData,
        }
    }