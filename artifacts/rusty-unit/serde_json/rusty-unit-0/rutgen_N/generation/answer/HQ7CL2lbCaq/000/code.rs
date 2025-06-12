// Answer 0

#[test]
fn test_deserialize_seq_with_array() {
    use serde::de::{Visitor, Deserializer};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor {
        value: Option<Vec<i32>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
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

    let result: Result<Vec<i32>, Error> = array_value.deserialize_seq(TestVisitor { value: None });
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_seq_with_non_array() {
    use serde::de::{Visitor, Deserializer};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array")
        }

        fn visit_seq<A>(self, _: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let non_array_value = Value::Number(serde_json::Number::from(42));

    let result: Result<(), Error> = non_array_value.deserialize_seq(TestVisitor);
    assert!(result.is_err());
}

