// Answer 0

#[test]
fn test_deserialize_bool_valid_true() {
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean value")
        }
    }

    let value = Value::Bool(true);
    let visitor = BoolVisitor;
    let result = value.deserialize_bool(visitor);
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_bool_valid_false() {
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean value")
        }
    }

    let value = Value::Bool(false);
    let visitor = BoolVisitor;
    let result = value.deserialize_bool(visitor);
    assert_eq!(result.unwrap(), false);
}

#[test]
#[should_panic]
fn test_deserialize_bool_invalid_type() {
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean value")
        }
    }

    let value = Value::Number(Number { n: 0.0 }); // Invalid type
    let visitor = BoolVisitor;
    let _result = value.deserialize_bool(visitor).unwrap(); // This should panic
}

