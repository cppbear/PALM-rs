// Answer 0

#[test]
fn test_deserialize_map_invalid_type() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any map")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Value::String("not an object".to_owned()); // self does not match Value::Object
    let visitor = TestVisitor;

    let result: Result<(), _> = value.deserialize_map(visitor);
    
    assert!(result.is_err());
}

#[test]
fn test_deserialize_map_other_value() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any map")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Value::Array(vec![]); // self does not match Value::Object
    let visitor = TestVisitor;

    let result: Result<(), _> = value.deserialize_map(visitor);
    
    assert!(result.is_err());
}

