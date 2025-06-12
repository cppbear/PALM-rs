// Answer 0

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct TestVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<Value>;

        fn visit_none(self) -> Result<Self::Value, de::Error> {
            self.visited = true;
            Ok(None)
        }

        fn visit_some(self, _: &Value) -> Result<Self::Value, de::Error> {
            Ok(Some(Value::Null))
        }
    }

    #[test]
    fn test_deserialize_option_value_null() {
        let value = Value::Null;
        let visitor = TestVisitor { visited: false };
        let result = value.deserialize_option(visitor);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_deserialize_option_value_not_null() {
        let value = Value::from("some data");
        let visitor = TestVisitor { visited: false };
        let result = value.deserialize_option(visitor);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Value::Null));
    }
}

