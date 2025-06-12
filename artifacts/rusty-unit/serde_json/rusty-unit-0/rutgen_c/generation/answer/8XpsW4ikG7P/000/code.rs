// Answer 0

#[test]
fn test_deserialize_seq_valid_array() {
    struct TestVisitor {
        visited: Vec<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some(value) = seq.next_element()? {
                result.push(value);
            }
            Ok(result)
        }
    }

    let array_value = Value::Array(vec![
        Value::Bool(true),
        Value::Number(Number::from(42)),
        Value::String("test".to_string()),
    ]);

    let visitor = TestVisitor { visited: vec![] };
    let result = array_value.deserialize_seq(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![
        Value::Bool(true),
        Value::Number(Number::from(42)),
        Value::String("test".to_string()),
    ]);
}

#[test]
fn test_deserialize_seq_invalid_type() {
    struct TestVisitor {
        visited: Vec<Value>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(vec![])
        }
    }

    let not_array_value = Value::Bool(false);
    let visitor = TestVisitor { visited: vec![] };
    let result = not_array_value.deserialize_seq(visitor);
    assert!(result.is_err());
}

