fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = tri!(visitor.visit_seq(&mut self));
        tri!(self.end());
        Ok(value)
    }