// Answer 0

#[test]
fn test_deserialize_enum_string_variant() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct TestVisitor {
        expected: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = &'static str;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string matching one of the variants")
        }

        fn visit_enum<V>(self, deserializer: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::EnumAccess<'de>,
        {
            let (variant, _value): (_, Option<Value>) = deserializer.into_serde()?;
            assert_eq!(variant, self.expected);
            Ok(variant)
        }
    }

    let value = Value::String("variant1".to_string());
    let visitor = TestVisitor { expected: "variant1" };
    let result: Result<&'static str, serde::de::Error> = value.deserialize_enum("TestEnum", &["variant1", "variant2"], visitor);
    
    assert_eq!(result, Ok("variant1"));
}

#[test]
#[should_panic(expected = "invalid type")]
fn test_deserialize_enum_invalid_type() {
    use serde_json::Value;
    
    let value = Value::Number(serde_json::Number::from(42)); // invalid type
    let visitor = TestVisitor { expected: "variant1" };
    let _result: Result<&'static str, serde::de::Error> = value.deserialize_enum("TestEnum", &["variant1", "variant2"], visitor);
}

