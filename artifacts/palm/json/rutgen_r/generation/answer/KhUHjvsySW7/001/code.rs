// Answer 0

#[test]
fn test_visit_with_i64() {
    use serde::de::{self, Visitor};
    use serde::Deserialize;

    struct MockVisitor {
        value: Option<i64>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = i64;

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::custom("expected i64 but found f64"))
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::custom("expected i64 but found u64"))
        }
    }

    enum ParserNumber {
        F64(f64),
        U64(u64),
        I64(i64),
        #[cfg(feature = "arbitrary_precision")]
        String(String),
    }

    impl ParserNumber {
        fn visit<'de, V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
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
    }

    let parser_number = ParserNumber::I64(42);
    let visitor = MockVisitor { value: None };

    let result = parser_number.visit(visitor);
    assert_eq!(result.unwrap(), 42);
}

