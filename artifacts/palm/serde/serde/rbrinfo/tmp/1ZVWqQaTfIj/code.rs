fn deserialize_enum<V>(
        self,
        name: &str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = name;
        let _ = variants;
        visitor.visit_enum(self)
    }