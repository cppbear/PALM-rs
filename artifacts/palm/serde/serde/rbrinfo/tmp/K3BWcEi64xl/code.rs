pub fn new(value: u32) -> Self {
        U32Deserializer {
            value,
            marker: PhantomData,
        }
    }