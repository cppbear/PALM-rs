fn into_deserializer(self) -> U32Deserializer<E> {
        U32Deserializer::new(self)
    }