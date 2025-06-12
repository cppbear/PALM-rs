// Answer 0

#[test]
fn test_deserialize_map_with_object() {
    use serde::de::{Deserialize, Deserializer, Visitor};
    use serde_json::{Value, Error};
    use std::fmt;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object")
        }

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok("visited".to_string())
        }
    }

    let json_value = Value::Object(serde_json::map::Map::new());
    let result: Result<String, Error> = json_value.deserialize_map(TestVisitor);
    assert_eq!(result.unwrap(), "visited");
}

#[test]
fn test_deserialize_map_with_non_object() {
    use serde::de::{Deserialize, Deserializer, Visitor};
    use serde_json::{Value, Error};
    use std::fmt;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object")
        }

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok("visited".to_string())
        }
    }

    let json_value = Value::String("not an object".to_string());
    let result: Result<String, Error> = json_value.deserialize_map(TestVisitor);
    assert!(result.is_err());
}

