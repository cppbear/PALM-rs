fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match *self.content {
                Content::Seq(ref v) => visit_content_seq_ref(v, visitor),
                Content::Map(ref v) => visit_content_map_ref(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }