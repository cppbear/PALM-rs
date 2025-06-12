// Answer 0

#[test]
fn test_deserialize_map_with_object() {
    struct TestVisitor {
        value: Option<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Error>
        where
            M: MapAccess<'de>,
        {
            Ok(vec![Value::Null])
        }
    }

    let value = Value::Object(Map {
        map: Default::default(),
    });
    let visitor = TestVisitor { value: None };

    let result = value.deserialize_map(visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_map_with_non_object() {
    struct TestVisitor {
        value: Option<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Error>
        where
            M: MapAccess<'de>,
        {
            Ok(vec![Value::Null])
        }
    }

    let value = Value::Null;
    let visitor = TestVisitor { value: None };

    let result = value.deserialize_map(visitor);
    assert!(result.is_err());
}

