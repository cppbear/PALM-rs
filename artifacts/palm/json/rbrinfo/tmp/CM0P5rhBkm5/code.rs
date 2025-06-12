fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match tri!(self.parse_whitespace()) {
            Some(b'n') => {
                self.eat_char();
                tri!(self.parse_ident(b"ull"));
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }