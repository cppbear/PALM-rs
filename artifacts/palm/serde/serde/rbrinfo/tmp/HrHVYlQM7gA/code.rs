fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(FlatMapAccess {
            iter: self.0.iter(),
            pending_content: None,
            _marker: PhantomData,
        })
    }