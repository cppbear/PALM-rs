// Answer 0

#[test]
fn test_deserialize_any_bool_true() {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            assert_eq!(value, true);
            Ok(value)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(de::Error::custom("Expected a bool value, found unit"))
        }

        fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
            Err(de::Error::custom("Expected a bool value, found string"))
        }

        // Other necessary methods can return errors as they are not used in this test
        fn visit_number<E>(self, _: &de::Number) -> Result<Self::Value, E> {
            Err(de::Error::custom("Expected a bool value, found number"))
        }

        fn visit_array<E>(self, _: &[Self::Value]) -> Result<Self::Value, E> {
            Err(de::Error::custom("Expected a bool value, found array"))
        }

        fn visit_map<E>(self, _: &mut dyn de::MapAccess<'de>) -> Result<Self::Value, E> {
            Err(de::Error::custom("Expected a bool value, found object"))
        }
    }

    let value = Value::Bool(true);
    let visitor = BoolVisitor;

    let result = value.deserialize_any(visitor);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_any_bool_false() {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            assert_eq!(value, false);
            Ok(value)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(de::Error::custom("Expected a bool value, found unit"))
        }

        fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
            Err(de::Error::custom("Expected a bool value, found string"))
        }

        // Other necessary methods can return errors as they are not used in this test
        fn visit_number<E>(self, _: &de::Number) -> Result<Self::Value, E> {
            Err(de::Error::custom("Expected a bool value, found number"))
        }

        fn visit_array<E>(self, _: &[Self::Value]) -> Result<Self::Value, E> {
            Err(de::Error::custom("Expected a bool value, found array"))
        }

        fn visit_map<E>(self, _: &mut dyn de::MapAccess<'de>) -> Result<Self::Value, E> {
            Err(de::Error::custom("Expected a bool value, found object"))
        }
    }

    let value = Value::Bool(false);
    let visitor = BoolVisitor;

    let result = value.deserialize_any(visitor);
    assert_eq!(result, Ok(false));
}

