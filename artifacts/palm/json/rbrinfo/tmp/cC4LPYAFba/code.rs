fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Object(v) => v.deserialize_any(visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }