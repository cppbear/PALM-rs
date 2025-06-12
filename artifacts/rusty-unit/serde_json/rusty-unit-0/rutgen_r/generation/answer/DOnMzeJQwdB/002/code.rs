// Answer 0

#[test]
fn test_deserialize_bool_success() {
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean value")
        }
    }

    let value = Value::Bool(true);
    let result: Result<bool, Error> = value.deserialize_bool(TestVisitor);
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_bool_false() {
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean value")
        }
    }

    let value = Value::Bool(false);
    let result: Result<bool, Error> = value.deserialize_bool(TestVisitor);
    assert_eq!(result.unwrap(), false);
}

#[test]
fn test_deserialize_non_bool_type() {
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean value")
        }
    }

    let value = Value::Number(serde_json::Number::from(1));
    let result: Result<bool, Error> = value.deserialize_bool(TestVisitor);
    assert!(result.is_err());
}

