// Answer 0

#[test]
fn test_visit_parser_number_u64() {
    struct MockVisitor {
        value: Option<u64>,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u64;
        
        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E> {
            panic!("visit_f64 should not be called for u64")
        }

        fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E> {
            panic!("visit_i64 should not be called for u64")
        }

        #[cfg(feature = "arbitrary_precision")]
        fn visit_map<E>(self, _map: impl de::MapAccess<'de>) -> Result<Self::Value, E> {
            panic!("visit_map should not be called for u64")
        }
    }

    let value = ParserNumber::U64(42);

    let visitor = MockVisitor { value: None };
    let result = value.visit(visitor);

    assert_eq!(result, Ok(42));
}

#[test]
fn test_visit_parser_number_f64_should_panic() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u64;
        
        fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E> {
            panic!("visit_f64 should not be called")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E> {
            panic!("visit_i64 should not be called")
        }

        #[cfg(feature = "arbitrary_precision")]
        fn visit_map<E>(self, _map: impl de::MapAccess<'de>) -> Result<Self::Value, E> {
            panic!("visit_map should not be called")
        }
    }

    let value = ParserNumber::F64(3.14);

    let visitor = MockVisitor;
    let result = std::panic::catch_unwind(|| {
        value.visit(visitor).unwrap();
    });

    assert!(result.is_err());
}

#[test]
fn test_visit_parser_number_i64_should_panic() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u64;
        
        fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E> {
            panic!("visit_f64 should not be called")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E> {
            panic!("visit_i64 should not be called")
        }

        #[cfg(feature = "arbitrary_precision")]
        fn visit_map<E>(self, _map: impl de::MapAccess<'de>) -> Result<Self::Value, E> {
            panic!("visit_map should not be called")
        }
    }

    let value = ParserNumber::I64(-10);

    let visitor = MockVisitor;
    let result = std::panic::catch_unwind(|| {
        value.visit(visitor).unwrap();
    });

    assert!(result.is_err());
}

