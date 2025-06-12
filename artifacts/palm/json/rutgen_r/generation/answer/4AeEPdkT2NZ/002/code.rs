// Answer 0

#[test]
fn test_deserialize_enum_with_object() {
    use serde::{Deserialize, Deserializer};
    use serde_json::{Value, Error, Map};
    
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_enum<E>(self, deserializer: E) -> Result<Self::Value, E::Error>
        where
            E: serde::de::EnumAccess<'de>,
        {
            Ok(self.value.unwrap_or_else(|| String::from("default_value")))
        }
    }

    struct EnumDeserializer {
        variant: String,
        value: Option<Value>,
    }

    impl<'de> serde::de::EnumAccess<'de> for EnumDeserializer {
        type Error = Error;

        fn variant<V>(self, _name: &'static str) -> Result<(Self::Value, V), Self::Error>
        where
            V: serde::de::Deserializer<'de>,
        {
            Ok((self.variant, TestVisitor { value: self.value.map(|v| v.to_string()) }))
        }
    }

    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("variant1".to_string()));
    map.insert("key2".to_string(), Value::String("variant2".to_string()));
    
    let value = Value::Object(map);
    let variants = vec!["variant1", "variant2"];
    
    let result: Result<String, Error> = value.deserialize_enum("my_enum", &variants, TestVisitor { value: None });

    assert_eq!(result, Ok("default_value".to_string()));
}

#[test]
fn test_deserialize_enum_with_invalid_type() {
    use serde::{Deserialize, Deserializer};
    use serde_json::{Value, Error};

    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_enum<E>(self, _deserializer: E) -> Result<Self::Value, E::Error>
        where
            E: serde::de::EnumAccess<'de>,
        {
            Ok("expected_value".to_string())
        }
    }
    
    let value = Value::Number(serde_json::Number::from(123));

    let variants = vec!["variant1", "variant2"];
    
    let result: Result<String, Error> = value.deserialize_enum("my_enum", &variants, TestVisitor { value: None });

    assert!(result.is_err());
}

