fn new(value: Cow<'de, str>) -> Self {
        BorrowedCowStrDeserializer { value }
    }