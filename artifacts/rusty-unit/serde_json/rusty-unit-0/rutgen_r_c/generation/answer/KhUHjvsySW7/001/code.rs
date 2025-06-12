// Answer 0

#[test]
fn test_visit_i64() {
    struct TestVisitor {
        value: Option<i64>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i64;

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, E> 
        where
            V: de::MapAccess<'de>,
        {
            unimplemented!()
        }
    }

    let num = ParserNumber::I64(42);
    let visitor = TestVisitor { value: None };

    let result = num.visit(visitor);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_visit_i64_neg() {
    struct TestVisitor {
        value: Option<i64>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i64;

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, E> 
        where
            V: de::MapAccess<'de>,
        {
            unimplemented!()
        }
    }

    let num = ParserNumber::I64(-10);
    let visitor = TestVisitor { value: None };

    let result = num.visit(visitor);
    assert_eq!(result.unwrap(), -10);
}

#[test]
fn test_visit_i64_zero() {
    struct TestVisitor {
        value: Option<i64>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i64;

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, E> 
        where
            V: de::MapAccess<'de>,
        {
            unimplemented!()
        }
    }

    let num = ParserNumber::I64(0);
    let visitor = TestVisitor { value: None };

    let result = num.visit(visitor);
    assert_eq!(result.unwrap(), 0);
}

