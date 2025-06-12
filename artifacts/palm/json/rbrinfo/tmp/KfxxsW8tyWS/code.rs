fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::String(v) => visitor.visit_string(v),
            Value::Array(v) => visit_array(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }