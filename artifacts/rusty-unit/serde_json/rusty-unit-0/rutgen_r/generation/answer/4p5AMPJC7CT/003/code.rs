// Answer 0

#[test]
fn test_deserialize_struct_with_array() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<V>(self, seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            let mut seq_access = seq;
            while let Some(value) = seq_access.next_element()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let input = Value::Array(vec![Value::Number(serde_json::Number::from(1)), Value::Number(serde_json::Number::from(2)), Value::Number(serde_json::Number::from(3))]);
    
    let result: Result<Vec<i32>, Error> = input.deserialize_struct("TestArray", &["field1", "field2", "field3"], TestVisitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_struct_with_empty_array() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<V>(self, seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            let mut seq_access = seq;
            while let Some(value) = seq_access.next_element()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let input = Value::Array(vec![]);
    
    let result: Result<Vec<i32>, Error> = input.deserialize_struct("TestArray", &[], TestVisitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![]);
}

