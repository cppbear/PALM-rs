fn deserialize_newtype_struct<V>(
            self,
            _name: &str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::Newtype(v) => visitor.visit_newtype_struct(ContentDeserializer::new(*v)),
                _ => visitor.visit_newtype_struct(self),
            }
        }