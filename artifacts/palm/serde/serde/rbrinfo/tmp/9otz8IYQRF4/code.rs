pub fn new(value: &'a [u8]) -> Self {
        BytesDeserializer {
            value,
            marker: PhantomData,
        }
    }