// Answer 0

#[test]
fn test_deserialize_option_none() {
    struct TestVisitor;
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
    let result = value.deserialize_option(TestVisitor);
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_deserialize_option_some() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Value>;
        fn visit_none(self) -> Result<Self::Value, Error> {
            Ok(None)
        }
        fn visit_some(self, value: Value) -> Result<Self::Value, Error> {
            Ok(Some(value))
        }
    }

    let value = Value::Bool(true);
    let result = value.deserialize_option(TestVisitor);
    assert_eq!(result.unwrap(), Some(Value::Bool(true)));
}

#[test]
fn test_deserialize_option_invalid() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Value>;
        fn visit_none(self) -> Result<Self::Value, Error> {
            Ok(None)
        }
        fn visit_some(self, _value: Value) -> Result<Self::Value, Error> {
            Err(Error) // Simulate an error on visit_some for demonstration
        }
    }

    let value = Value::Bool(false);
    let result = value.deserialize_option(TestVisitor);
    assert!(result.is_err());
}

