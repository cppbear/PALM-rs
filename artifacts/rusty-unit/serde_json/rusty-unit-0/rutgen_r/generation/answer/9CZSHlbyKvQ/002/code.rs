// Answer 0

#[test]
fn test_deserialize_bool_success() {
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value, Error> {
            Ok(value)
        }
        
        // other required methods can be left unimplemented for this test
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let value = serde_json::Value::Bool(true);
    let result = value.deserialize_bool(BoolVisitor);
    assert_eq!(result, Ok(true));
}

#[test]
#[should_panic(expected = "invalid type")]
fn test_deserialize_bool_invalid_type() {
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value, Error> {
            Ok(value)
        }
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean")
        }
    }

    let value = serde_json::Value::Number(serde_json::Number::from(1));
    let _ = value.deserialize_bool(BoolVisitor);
}

