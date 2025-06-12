// Answer 0

fn test_visit_f64() {
    struct TestVisitor {
        value: f64,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = f64;

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_u64 called instead of visit_f64"))
        }

        fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_i64 called instead of visit_f64"))
        }
    }

    let number = ParserNumber::F64(3.14);
    let visitor = TestVisitor { value: 3.14 };
    let result = number.visit(visitor).unwrap();
    assert_eq!(result, 3.14);
}

fn test_visit_u64_instead_of_f64() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = f64;

        fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_f64 called instead of visit_u64"))
        }

        fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E> {
            Ok(0.0)
        }

        fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_i64 called instead of visit_u64"))
        }
    }

    let number = ParserNumber::F64(3.14);
    let visitor = TestVisitor;
    let result = number.visit(visitor);
    assert!(result.is_err());
}

fn test_visit_i64_instead_of_f64() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = f64;

        fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_f64 called instead of visit_i64"))
        }

        fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E> {
            Err(de::Error::custom("visit_u64 called instead of visit_i64"))
        }

        fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E> {
            Ok(0.0)
        }
    }

    let number = ParserNumber::F64(3.14);
    let visitor = TestVisitor;
    let result = number.visit(visitor);
    assert!(result.is_err());
}

