fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::Seq(v) => visit_content_seq(v, visitor),
                Content::Map(v) => visit_content_map(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }