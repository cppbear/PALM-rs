fn deserialize_struct<V>(
        self,
        _: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(FlatStructAccess {
            iter: self.0.iter_mut(),
            pending_content: None,
            fields,
            _marker: PhantomData,
        })
    }