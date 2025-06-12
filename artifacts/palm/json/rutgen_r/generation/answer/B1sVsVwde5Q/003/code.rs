// Answer 0

#[test]
fn test_deserialize_struct_with_array() {
    use serde_json::Value;
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use std::fmt;

    struct TestVisitor {
        result: Vec<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let json_array = Value::Array(vec![
        Value::Number(serde_json::Number::from(1)),
        Value::Number(serde_json::Number::from(2)),
        Value::Number(serde_json::Number::from(3)),
    ]);

    let visitor = TestVisitor { result: Vec::new() };
    let result = json_array.deserialize_struct("TestStruct", &["field1"], visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_struct_with_empty_array() {
    use serde_json::Value;
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use std::fmt;

    struct TestVisitor {
        result: Vec<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let json_array = Value::Array(vec![]);

    let visitor = TestVisitor { result: Vec::new() };
    let result = json_array.deserialize_struct("TestStruct", &["field1"], visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![]);
}

