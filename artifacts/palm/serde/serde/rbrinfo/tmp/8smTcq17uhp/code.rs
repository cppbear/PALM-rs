fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Cow::Borrowed(string) => visitor.visit_str(string),
            Cow::Owned(string) => visitor.visit_string(string),
        }
    }