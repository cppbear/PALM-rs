pub fn new(read: R) -> Self {
        let offset = read.byte_offset();
        StreamDeserializer {
            de: Deserializer::new(read),
            offset,
            failed: false,
            output: PhantomData,
            lifetime: PhantomData,
        }
    }