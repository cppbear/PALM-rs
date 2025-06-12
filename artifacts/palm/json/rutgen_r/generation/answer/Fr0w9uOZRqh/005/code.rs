// Answer 0

#[test]
fn test_deserialize_any_with_bool_true() {
    use serde_json::Value;
    use serde::de::Visitor;
    use serde::de::Deserialize;

    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            assert_eq!(value, true);
            Ok(value)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            panic!("Expected a boolean value, but found null.");
        }

        fn visit_string<E>(self, _value: String) -> Result<Self::Value, E> {
            panic!("Expected a boolean value, but found a string.");
        }

        fn visit_array<E>(self, _value: &[Self::Value]) -> Result<Self::Value, E> {
            panic!("Expected a boolean value, but found an array.");
        }

        fn visit_object<E>(self, _value: &serde_json::Map<String, Value>) -> Result<Self::Value, E> {
            panic!("Expected a boolean value, but found an object.");
        }
    }

    let value = Value::Bool(true);
    let result = value.deserialize_any(BoolVisitor);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_any_with_bool_false() {
    use serde_json::Value;
    use serde::de::Visitor;

    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            assert_eq!(value, false);
            Ok(value)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            panic!("Expected a boolean value, but found null.");
        }

        fn visit_string<E>(self, _value: String) -> Result<Self::Value, E> {
            panic!("Expected a boolean value, but found a string.");
        }

        fn visit_array<E>(self, _value: &[Self::Value]) -> Result<Self::Value, E> {
            panic!("Expected a boolean value, but found an array.");
        }

        fn visit_object<E>(self, _value: &serde_json::Map<String, Value>) -> Result<Self::Value, E> {
            panic!("Expected a boolean value, but found an object.");
        }
    }

    let value = Value::Bool(false);
    let result = value.deserialize_any(BoolVisitor);
    assert_eq!(result, Ok(false));
}

