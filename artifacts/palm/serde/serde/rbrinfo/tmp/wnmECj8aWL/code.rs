fn into_deserializer(self) -> StringDeserializer<E> {
        StringDeserializer::new(self)
    }