fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::String(v) => visitor.visit_string(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }