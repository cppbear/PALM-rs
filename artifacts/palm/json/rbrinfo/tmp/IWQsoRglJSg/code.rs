fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        BorrowedCowStrDeserializer::new(self.key).deserialize_any(visitor)
    }