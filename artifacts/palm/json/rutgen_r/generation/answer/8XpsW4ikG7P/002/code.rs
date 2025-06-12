// Answer 0

#[test]
fn test_deserialize_seq_with_valid_array() {
    use serde_json::Value;
    use serde::de::{self, Visitor};
    
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let array_value = Value::Array(vec![Value::Number(serde_json::Number::from(1)), 
                                         Value::Number(serde_json::Number::from(2)), 
                                         Value::Number(serde_json::Number::from(3))]);

    let result: Result<Vec<i32>, _> = array_value.deserialize_seq(TestVisitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deserialize_seq_with_non_array() {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            // Just a placeholder for test purpose
            Ok(Vec::new())
        }
    }

    let non_array_value = Value::Object(serde_json::Map::new());

    let _ = non_array_value.deserialize_seq(TestVisitor);
}

