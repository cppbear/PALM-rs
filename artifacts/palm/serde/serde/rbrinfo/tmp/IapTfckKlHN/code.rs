fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match *self.content {
                Content::Seq(ref v) => visit_content_seq_ref(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }