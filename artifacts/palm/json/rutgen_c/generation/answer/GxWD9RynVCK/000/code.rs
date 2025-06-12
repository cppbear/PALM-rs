// Answer 0

#[test]
fn test_deserialize_tuple_struct_empty() {
    struct TestVisitor;
    
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple struct")
        }

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, serde::de::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            Ok(vec![])
        }
    }

    let value = Value::Array(vec![]);
    let deserializer = &value;

    let result: Result<Vec<i32>, Error> = deserializer.deserialize_tuple_struct("Test", 0, TestVisitor);
    assert_eq!(result.unwrap(), vec![]);
}

#[test]
fn test_deserialize_tuple_struct_with_elements() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple struct")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, serde::de::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let value = Value::Array(vec![Value::Number(Number::from(1)), Value::Number(Number::from(2))]);
    let deserializer = &value;

    let result: Result<Vec<i32>, Error> = deserializer.deserialize_tuple_struct("Test", 2, TestVisitor);
    assert_eq!(result.unwrap(), vec![1, 2]);
}

#[test]
fn test_deserialize_tuple_struct_with_mismatched_length() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple struct")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, serde::de::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let value = Value::Array(vec![Value::Number(Number::from(1))]);
    let deserializer = &value;

    let result: Result<Vec<i32>, Error> = deserializer.deserialize_tuple_struct("Test", 2, TestVisitor);
    assert!(result.is_err());
}

