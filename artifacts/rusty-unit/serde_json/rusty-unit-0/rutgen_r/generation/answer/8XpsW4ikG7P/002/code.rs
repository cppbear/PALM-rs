// Answer 0

#[test]
fn test_deserialize_seq_valid_array() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let array_value = Value::Array(vec![Value::Number(serde_json::Number::from(1)), 
                                         Value::Number(serde_json::Number::from(2)), 
                                         Value::Number(serde_json::Number::from(3))]);

    let result = array_value.deserialize_seq(TestVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deserialize_seq_invalid_type() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<S>(self, _: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            Err(de::Error::custom("This shouldn't be called"))
        }
    }

    let number_value = Value::Number(serde_json::Number::from(1));

    let _ = number_value.deserialize_seq(TestVisitor);
}

