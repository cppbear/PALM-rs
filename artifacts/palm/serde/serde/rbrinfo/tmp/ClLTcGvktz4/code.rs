pub fn new(value: String) -> Self {
        StringDeserializer {
            value,
            marker: PhantomData,
        }
    }