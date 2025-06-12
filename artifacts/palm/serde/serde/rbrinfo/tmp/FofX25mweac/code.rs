pub fn new(value: &'a str) -> Self {
        StrDeserializer {
            value,
            marker: PhantomData,
        }
    }