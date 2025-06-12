fn visit_content_seq_ref<'a, 'de, V, E>(
        content: &'a [Content<'de>],
        visitor: V,
    ) -> Result<V::Value, E>
    where
        V: Visitor<'de>,
        E: de::Error,
    {
        let mut seq_visitor = SeqDeserializer::new(content.iter());
        let value = tri!(visitor.visit_seq(&mut seq_visitor));
        tri!(seq_visitor.end());
        Ok(value)
    }