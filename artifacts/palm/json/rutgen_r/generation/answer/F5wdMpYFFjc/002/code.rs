// Answer 0

#[test]
fn test_deserialize_str_valid() {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct StringVisitor;

    impl<'de> Visitor<'de> for StringVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, de::Error> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a borrowed string")
        }
    }

    let value = Value::String(String::from("test string"));
    let result: Result<&str, _> = value.deserialize_str(StringVisitor);
    assert_eq!(result, Ok("test string"));
}

#[test]
#[should_panic]
fn test_deserialize_str_invalid_type() {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct StringVisitor;

    impl<'de> Visitor<'de> for StringVisitor {
        type Value = &'de str;

        fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value, de::Error> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a borrowed string")
        }
    }

    let value = Value::Number(serde_json::Number::from(42)); // Not a Value::String
    let _result: Result<&str, _> = value.deserialize_str(StringVisitor);
}

