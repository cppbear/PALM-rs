// Answer 0

#[test]
fn test_tuple_variant_with_non_empty_array() {
    use serde_json::{Value, Error, de::Visitor};

    struct TestVisitor {
        value: Option<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Result<Value, Error>;

        fn visit_array<V>(self, mut values: V) -> Self::Value
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut array_values = Vec::new();
            while let Some(value) = values.next_element()? {
                array_values.push(value);
            }
            Ok(Value::Array(array_values))
        }

        fn visit_unit(self) -> Self::Value {
            Ok(Value::Null)
        }
    }

    let value = Some(Value::Array(vec![Value::Number(1.into()), Value::Bool(true)]));

    let result = tuple_variant(value, TestVisitor { value: None });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Array(vec![Value::Number(1.into()), Value::Bool(true)]));
}

#[test]
fn test_tuple_variant_with_non_array_value() {
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Result<Value, Error>;

        fn visit_array<V>(self, _: V) -> Self::Value
        where
            V: serde::de::SeqAccess<'de>,
        {
            unreachable!()
        }

        fn visit_unit(self) -> Self::Value {
            Ok(Value::Null)
        }
    }

    let value = Some(Value::String("not an array".to_string()));

    let result = tuple_variant(value, TestVisitor);

    assert!(result.is_err());
}

#[test]
fn test_tuple_variant_with_none() {
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Result<Value, Error>;

        fn visit_array<V>(self, _: V) -> Self::Value
        where
            V: serde::de::SeqAccess<'de>,
        {
            unreachable!()
        }

        fn visit_unit(self) -> Self::Value {
            Ok(Value::Null)
        }
    }

    let value: Option<Value> = None;

    let result = tuple_variant(value, TestVisitor);

    assert!(result.is_err());
}

#[test]
fn test_tuple_variant_with_empty_array() {
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Result<Value, Error>;

        fn visit_array<V>(self, _: V) -> Self::Value
        where
            V: serde::de::SeqAccess<'de>,
        {
            unreachable!()
        }

        fn visit_unit(self) -> Self::Value {
            Ok(Value::Null)
        }
    }

    let value = Some(Value::Array(vec![]));

    let result = tuple_variant(value, TestVisitor);

    assert_eq!(result, Ok(Value::Null));
}

