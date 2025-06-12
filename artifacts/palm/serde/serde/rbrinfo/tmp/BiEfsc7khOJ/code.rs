fn visit_content_map<'de, V, E>(
        content: Vec<(Content<'de>, Content<'de>)>,
        visitor: V,
    ) -> Result<V::Value, E>
    where
        V: Visitor<'de>,
        E: de::Error,
    {
        let mut map_visitor = MapDeserializer::new(content.into_iter());
        let value = tri!(visitor.visit_map(&mut map_visitor));
        tri!(map_visitor.end());
        Ok(value)
    }