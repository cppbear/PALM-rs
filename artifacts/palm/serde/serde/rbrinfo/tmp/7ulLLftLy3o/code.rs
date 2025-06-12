fn from(self) -> Self::Deserializer {
        BytesDeserializer::new(self)
    }