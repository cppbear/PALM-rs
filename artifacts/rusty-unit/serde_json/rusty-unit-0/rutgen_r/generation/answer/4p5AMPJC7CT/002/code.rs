// Answer 0

#[test]
fn test_deserialize_struct_with_object() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct or map")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(String::from("visited_map"))
        }
    }

    let value = Value::Object(serde_json::map::Map::new());

    let result: Result<String, Error> = value.deserialize_struct("TestStruct", &["field1", "field2"], TestVisitor);
    assert_eq!(result.unwrap(), "visited_map");
}

#[test]
fn test_deserialize_struct_with_non_object() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct or map")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(String::from("visited_map"))
        }
    }

    let value = Value::Array(vec![Value::String("item".to_string())]);

    let result: Result<String, Error> = value.deserialize_struct("TestStruct", &["field1", "field2"], TestVisitor);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_deserialize_struct_with_invalid_type() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct or map")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(String::from("visited_map"))
        }
    }

    let value = Value::String("invalid".to_string());

    let _ = value.deserialize_struct("TestStruct", &["field1", "field2"], TestVisitor);
}

