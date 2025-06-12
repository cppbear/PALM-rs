// Answer 0

#[test]
fn test_visit_array_ref_empty() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let array: &[Value] = &[];
    let result = visit_array_ref(array, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_array_ref_non_empty() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn visit_seq<V>(self, seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            let mut result = Vec::new();
            let mut seq_access = seq;
            while let Some(value) = seq_access.next_element::<Value>()? {
                result.push(value);
            }
            Ok(result)
        }
    }

    let array: &[Value] = &[Value::Number(Number::from(1)), Value::Number(Number::from(2))];
    let result = visit_array_ref(array, TestVisitor);
    assert!(result.is_ok());
    let values = result.unwrap();
    assert_eq!(values.len(), 2);
}

#[test]
#[should_panic(expected = "invalid_length")]
fn test_visit_array_ref_invalid_length() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn visit_seq<V>(self, seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(vec![Value::Number(Number::from(1))])
        }
    }

    let array: &[Value] = &[Value::Number(Number::from(1)), Value::Number(Number::from(2))];
    let _ = visit_array_ref(array, TestVisitor).unwrap();
}

