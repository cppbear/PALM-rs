// Answer 0

#[test]
fn test_struct_variant_with_object() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    let input_value = Some(Value::Object(serde_json::Map::new()));
    let result = struct_variant(input_value, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_struct_variant_with_other_value() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    let input_value = Some(Value::String("not an object".to_string()));
    let result = struct_variant(input_value, TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_struct_variant_with_none() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    let input_value = None;
    let result = struct_variant(input_value, TestVisitor);
    assert!(result.is_err());
}

