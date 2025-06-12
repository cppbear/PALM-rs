fn visit_map<V>(self, visitor: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            ContentVisitor::new()
                .visit_map(visitor)
                .map(TagOrContent::Content)
        }