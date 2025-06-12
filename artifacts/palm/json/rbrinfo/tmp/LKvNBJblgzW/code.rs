fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        tri!(self.ignore_value());
        visitor.visit_unit()
    }