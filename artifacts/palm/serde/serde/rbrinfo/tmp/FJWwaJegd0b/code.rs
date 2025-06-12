fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::Map(v) => visit_content_map(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }