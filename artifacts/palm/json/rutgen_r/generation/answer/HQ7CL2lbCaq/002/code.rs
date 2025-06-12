// Answer 0

#[test]
fn test_deserialize_seq_with_valid_array() {
    use serde_json::Value;
    use serde::de::Visitor;
    use serde::Deserialize;

    struct ArrayVisitor;

    impl<'de> Visitor<'de> for ArrayVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                result.push(value);
            }
            Ok(result)
        }
    }

    let json_array = Value::Array(vec![Value::Number(serde_json::Number::from(1)), 
                                        Value::Number(serde_json::Number::from(2)), 
                                        Value::Number(serde_json::Number::from(3))]);

    let result: Result<Vec<i32>, _> = json_array.deserialize_seq(ArrayVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
fn test_deserialize_seq_with_non_array_value() {
    use serde_json::Value;
    use serde::de::Visitor;
    use serde::Deserialize;

    struct ArrayVisitor;

    impl<'de> Visitor<'de> for ArrayVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                result.push(value);
            }
            Ok(result)
        }
    }

    let json_value = Value::Bool(true); // Not an array

    let _result: Result<Vec<i32>, _> = json_value.deserialize_seq(ArrayVisitor);
}

