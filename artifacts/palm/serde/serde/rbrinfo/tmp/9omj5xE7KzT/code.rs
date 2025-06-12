fn into_deserializer(self) -> StrDeserializer<'a, E> {
        StrDeserializer::new(self)
    }