// Answer 0

#[test]
fn test_visit_f64() {
    struct TestVisitor {
        value: Option<f64>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = f64;

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected f64"))
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected f64"))
        }
    }

    let number = ParserNumber::F64(3.14);
    let result = number.visit(TestVisitor { value: None });
    assert_eq!(result.unwrap(), 3.14);
}

#[test]
fn test_visit_u64() {
    struct TestVisitor {
        value: Option<u64>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = u64;

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected u64"))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected u64"))
        }
    }

    let number = ParserNumber::U64(42);
    let result = number.visit(TestVisitor { value: None });
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_visit_i64() {
    struct TestVisitor {
        value: Option<i64>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i64;

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected i64"))
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected i64"))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    let number = ParserNumber::I64(-7);
    let result = number.visit(TestVisitor { value: None });
    assert_eq!(result.unwrap(), -7);
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_visit_string() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected String"))
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected String"))
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected String"))
        }

        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            Ok("test".to_string())
        }
    }

    let number = ParserNumber::String("123".to_string());
    let result = number.visit(TestVisitor { value: None });
    assert_eq!(result.unwrap(), "test".to_string());
}

