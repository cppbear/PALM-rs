// Answer 0

#[test]
fn test_deserialize_option_some() {
    struct TestVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<&'de Value>;

        fn visit_none(self) -> Result<Self::Value, Error> {
            Ok(None)
        }

        fn visit_some(self, value: &'de Value) -> Result<Self::Value, Error> {
            self.visited = true;
            Ok(Some(value))
        }
    }

    let value = Value::Bool(true);
    let visitor = TestVisitor { visited: false };
    
    let result = (&value).deserialize_option(visitor).unwrap();
    assert!(result.is_some());
    assert!(result.unwrap() == &value);
}

#[test]
fn test_deserialize_option_none() {
    struct TestVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<&'de Value>;

        fn visit_none(self) -> Result<Self::Value, Error> {
            Ok(None)
        }

        fn visit_some(self, value: &'de Value) -> Result<Self::Value, Error> {
            self.visited = true;
            Ok(Some(value))
        }
    }

    let value = Value::Null;
    let visitor = TestVisitor { visited: false };
    
    let result = (&value).deserialize_option(visitor).unwrap();
    assert!(result.is_none());
}

