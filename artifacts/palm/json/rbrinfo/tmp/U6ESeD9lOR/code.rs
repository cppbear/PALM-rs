fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        // Map keys cannot be null.
        visitor.visit_some(self)
    }