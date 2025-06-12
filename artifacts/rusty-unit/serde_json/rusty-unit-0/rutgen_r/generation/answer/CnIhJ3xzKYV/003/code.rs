// Answer 0

#[test]
fn test_deserialize_any_success() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use std::collections::HashMap;

    struct MockVisitor {
        value: Option<HashMap<String, Value>>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = HashMap<String, Value>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<T>(self, mut visitor: T) -> Result<Self::Value, T::Error>
        where
            T: serde::de::MapAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some((key, value)) = visitor.next_entry::<String, Value>()? {
                map.insert(key, value);
            }
            Ok(map)
        }
    }

    let input = serde_json::json!({});
    let deserializer = serde_json::Deserializer::from_value(input);

    let visitor = MockVisitor { value: None };
    let result: Result<HashMap<String, Value>, _> = deserializer.deserialize_any(visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_deserialize_any_invalid_length() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use std::collections::HashMap;

    struct MockVisitor {
        // This is intentionally set up to cause an invalid length
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = HashMap<String, Value>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<T>(self, mut visitor: T) -> Result<Self::Value, T::Error>
        where
            T: serde::de::MapAccess<'de>,
        {
            // Returning a successful result but not consuming all items
            let mut map = HashMap::new();
            let key = visitor.next_key::<String>()?.ok_or_else(|| de::Error::custom("no key"))?;
            let value = visitor.next_value::<Value>()?;
            map.insert(key, value);
            Ok(map)
        }
    }

    let input = serde_json::json!({ "key": "value" });
    let deserializer = serde_json::Deserializer::from_value(input);
    let visitor = MockVisitor {};

    let result: Result<HashMap<String, Value>, _> = deserializer.deserialize_any(visitor);

    assert!(result.is_err());
}

