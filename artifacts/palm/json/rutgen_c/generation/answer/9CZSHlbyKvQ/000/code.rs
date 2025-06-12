// Answer 0

#[test]
fn test_deserialize_bool_valid() {
    struct MockVisitor {
        value: Option<bool>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    let value = Value::Bool(true);
    let visitor = MockVisitor { value: None };
    let result = value.deserialize_bool(visitor);
    assert_eq!(result, Ok(true));
}

#[test]
fn test_deserialize_bool_invalid() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = bool;

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            unimplemented!()
        }
    }

    let value = Value::Null;
    let visitor = MockVisitor;
    let result = value.deserialize_bool(visitor);
    assert!(result.is_err());
}

