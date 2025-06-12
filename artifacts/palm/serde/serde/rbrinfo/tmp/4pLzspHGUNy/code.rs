fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let v = tri!(visitor.visit_seq(&mut self));
        tri!(self.end());
        Ok(v)
    }