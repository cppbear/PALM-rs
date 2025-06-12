fn into_deserializer(self) -> Self::Deserializer {
            ContentRefDeserializer::new(self)
        }