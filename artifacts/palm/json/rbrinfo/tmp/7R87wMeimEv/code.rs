fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self.len();
        let mut deserializer = MapDeserializer::new(self);
        let map = tri!(visitor.visit_map(&mut deserializer));
        let remaining = deserializer.iter.len();
        if remaining == 0 {
            Ok(map)
        } else {
            Err(serde::de::Error::invalid_length(
                len,
                &"fewer elements in map",
            ))
        }
    }