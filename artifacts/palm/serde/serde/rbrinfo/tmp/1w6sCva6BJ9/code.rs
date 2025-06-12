fn into_deserializer(self) -> BytesDeserializer<'a, E> {
        BytesDeserializer::new(self)
    }