// Answer 0

#[test]
fn test_deserialize_option_some() {
    struct TestVisitor {
        visited: Option<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Value>;

        fn visit_none(self) -> Result<Self::Value, Error> {
            Ok(None)
        }

        fn visit_some(self, value: Value) -> Result<Self::Value, Error> {
            Ok(Some(value))
        }
    }

    let value = Value::String(String::from("some value"));
    let visitor = TestVisitor { visited: None };

    let result = value.deserialize_option(visitor).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), value);
}

#[test]
fn test_deserialize_option_null() {
    struct TestVisitor {
        visited: Option<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Value>;

        fn visit_none(self) -> Result<Self::Value, Error> {
            Ok(None)
        }

        fn visit_some(self, value: Value) -> Result<Self::Value, Error> {
            Ok(Some(value))
        }
    }

    let value = Value::Null;
    let visitor = TestVisitor { visited: None };

    let result = value.deserialize_option(visitor).unwrap();
    assert!(result.is_none());
}

