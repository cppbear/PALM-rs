fn visit_content_seq<'de, V, E>(content: Vec<Content<'de>>, visitor: V) -> Result<V::Value, E>
    where
        V: Visitor<'de>,
        E: de::Error,
    {
        let mut seq_visitor = SeqDeserializer::new(content.into_iter());
        let value = tri!(visitor.visit_seq(&mut seq_visitor));
        tri!(seq_visitor.end());
        Ok(value)
    }