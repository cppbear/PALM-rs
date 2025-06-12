fn visit_content_map_ref<'a, 'de, V, E>(
        content: &'a [(Content<'de>, Content<'de>)],
        visitor: V,
    ) -> Result<V::Value, E>
    where
        V: Visitor<'de>,
        E: de::Error,
    {
        fn content_ref_deserializer_pair<'a, 'de>(
            (k, v): &'a (Content<'de>, Content<'de>),
        ) -> (&'a Content<'de>, &'a Content<'de>) {
            (k, v)
        }

        let map = content.iter().map(content_ref_deserializer_pair);
        let mut map_visitor = MapDeserializer::new(map);
        let value = tri!(visitor.visit_map(&mut map_visitor));
        tri!(map_visitor.end());
        Ok(value)
    }