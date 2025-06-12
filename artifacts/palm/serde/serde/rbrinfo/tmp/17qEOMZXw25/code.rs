fn from(self) -> Self::Deserializer {
        StrDeserializer {
            value: self,
            marker: PhantomData,
        }
    }