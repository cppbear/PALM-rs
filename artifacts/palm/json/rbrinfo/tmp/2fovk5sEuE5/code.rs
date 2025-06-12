fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.de.eat_char();
        self.de.scratch.clear();
        match tri!(self.de.read.parse_str(&mut self.de.scratch)) {
            Reference::Borrowed(s) => visitor.visit_borrowed_str(s),
            Reference::Copied(s) => visitor.visit_str(s),
        }
    }