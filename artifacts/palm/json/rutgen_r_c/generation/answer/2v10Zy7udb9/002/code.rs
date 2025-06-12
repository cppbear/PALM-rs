// Answer 0

#[test]
fn test_deserialize_option_null() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Value>;

        fn visit_none(self) -> Result<Self::Value, Error> {
            Ok(None)
        }

        fn visit_some(self, value: Value) -> Result<Self::Value, Error> {
            Ok(Some(value))
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an option value")
        }
    }

    let value = Value::Null;
    let visitor = TestVisitor;
    let result = value.deserialize_option(visitor).unwrap();

    assert_eq!(result, None);
}

#[test]
fn test_deserialize_option_some() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Value>;

        fn visit_none(self) -> Result<Self::Value, Error> {
            panic!("visit_none should not be called");
        }

        fn visit_some(self, value: Value) -> Result<Self::Value, Error> {
            Ok(Some(value))
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an option value")
        }
    }

    let value = Value::Bool(true);
    let visitor = TestVisitor;
    let result = value.deserialize_option(visitor).unwrap();

    assert_eq!(result, Some(Value::Bool(true)));
}

