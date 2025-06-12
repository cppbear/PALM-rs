// Answer 0

#[test]
fn test_struct_variant_with_valid_object() {
    use serde::de::{self, Visitor};
    use serde_json::{self, Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    let input = Some(Value::Object(serde_json::Map::new()));
    let result = struct_variant(input, &["field1", "field2"], TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_struct_variant_with_invalid_type() {
    use serde::de::{self, Visitor};
    use serde_json::{self, Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(Value::Null)
        }
    }

    let input = Some(Value::String("not an object".to_owned()));
    let result = struct_variant(input, &["field1", "field2"], TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_struct_variant_with_none() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    let input: Option<Value> = None;
    let result = struct_variant(input, &["field1", "field2"], TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_struct_variant_with_unit_variant() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok(Value::Object(serde_json::Map::new()))
        }
    }

    let input = Some(Value::Null);
    let result = struct_variant(input, &["field1", "field2"], TestVisitor);
    assert!(result.is_err());
}

