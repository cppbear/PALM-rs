fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::Seq(v) => visit_content_seq(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }