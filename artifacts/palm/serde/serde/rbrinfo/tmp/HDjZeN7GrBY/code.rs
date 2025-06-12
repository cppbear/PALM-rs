fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match *self.content {
                Content::Char(v) => visitor.visit_char(v),
                Content::String(ref v) => visitor.visit_str(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }