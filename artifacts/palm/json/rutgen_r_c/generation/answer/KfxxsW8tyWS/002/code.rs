// Answer 0

#[test]
fn test_deserialize_byte_buf_with_array() {
    use crate::value::Value;
    use crate::error::Error;
    use alloc::vec;

    struct VisitorImpl {
        result: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Vec<u8>;

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
            Ok(value.into_bytes())
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(vec![])
        }
    }

    let value_array = Value::Array(vec![
        Value::String("first".to_string()),
        Value::String("second".to_string()),
    ]);

    let visitor = VisitorImpl { result: None };
    let result = value_array.deserialize_byte_buf(visitor);

    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
#[should_panic]
fn test_deserialize_byte_buf_with_invalid_value() {
    use crate::value::Value;
    use crate::error::Error;
    
    struct VisitorImpl;

    impl<'de> Visitor<'de> for VisitorImpl {
        type Value = Vec<u8>;

        fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
            panic!("visit_string should not be called in this case.");
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            panic!("visit_seq should not be called in this case.");
        }
    }

    let value_string = Value::String("a string".to_string());
    let visitor = VisitorImpl;
    value_string.deserialize_byte_buf(visitor).unwrap();
}

