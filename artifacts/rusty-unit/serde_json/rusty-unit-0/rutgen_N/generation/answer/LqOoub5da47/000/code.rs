// Answer 0

#[test]
fn test_deserialize_option_none() {
    use serde::de::{self, Visitor};

    struct MockVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value, de::Error> {
            self.visited = true;
            Ok(None)
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, de::Error> {
            self.visited = true;
            Ok(Some(()))
        }
    }

    let value = serde_json::Value::Null;
    let visitor = MockVisitor { visited: false };
    let result = value.deserialize_option(visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_deserialize_option_some() {
    use serde::de::{self, Visitor};

    struct MockVisitor {
        visited: bool,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value, de::Error> {
            self.visited = true;
            Ok(None)
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, de::Error> {
            self.visited = true;
            Ok(Some(()))
        }
    }

    let value = serde_json::Value::Bool(true);
    let visitor = MockVisitor { visited: false };
    let result = value.deserialize_option(visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(()));
}

