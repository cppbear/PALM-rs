fn visit<'de, V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self {
            ParserNumber::F64(x) => visitor.visit_f64(x),
            ParserNumber::U64(x) => visitor.visit_u64(x),
            ParserNumber::I64(x) => visitor.visit_i64(x),
            #[cfg(feature = "arbitrary_precision")]
            ParserNumber::String(x) => visitor.visit_map(NumberDeserializer { number: x.into() }),
        }
    }