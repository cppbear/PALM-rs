fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match *self.content {
                Content::Unit => visitor.visit_unit(),
                _ => Err(self.invalid_type(&visitor)),
            }
        }