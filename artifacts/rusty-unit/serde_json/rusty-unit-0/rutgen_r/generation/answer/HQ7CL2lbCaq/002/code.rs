// Answer 0

#[derive(Debug)]
struct TestVisitor {
    value: Option<Vec<i32>>,
}

impl<'de> serde::de::Visitor<'de> for TestVisitor {
    type Value = Vec<i32>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an array of integers")
    }

    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
    where
        S: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(value) = seq.next_element()? {
            vec.push(value);
        }
        Ok(vec)
    }
}

#[test]
fn test_deserialize_seq_valid_array() {
    let value = serde_json::Value::Array(vec![
        serde_json::Value::Number(serde_json::Number::from(1)),
        serde_json::Value::Number(serde_json::Number::from(2)),
        serde_json::Value::Number(serde_json::Number::from(3)),
    ]);
    
    let visitor = TestVisitor { value: None };
    let result = value.deserialize_seq(visitor).unwrap();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_deserialize_seq_empty_array() {
    let value = serde_json::Value::Array(vec![]);
    
    let visitor = TestVisitor { value: None };
    let result = value.deserialize_seq(visitor).unwrap();
    assert_eq!(result, vec![]);
}

#[test]
fn test_deserialize_seq_invalid_type_panics() {
    let value = serde_json::Value::Bool(true);
    
    let visitor = TestVisitor { value: None };
    let result = value.deserialize_seq(visitor);
    assert!(result.is_err());
}

