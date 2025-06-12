fn into_deserializer(self) -> CowStrDeserializer<'a, E> {
        CowStrDeserializer::new(self)
    }