// Answer 0

#[test]
fn test_deserialize_struct_with_array() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let data = Value::Array(vec![Value::Number(1.into()), Value::Number(2.into()), Value::Number(3.into())]);
    let visitor = TestVisitor;
    let result: Result<Vec<i32>, Error> = data.deserialize_struct("TestStruct", &["field1", "field2", "field3"], visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_struct_with_empty_array() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let data = Value::Array(vec![]);
    let visitor = TestVisitor;
    let result: Result<Vec<i32>, Error> = data.deserialize_struct("TestStruct", &[], visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![]);
}

