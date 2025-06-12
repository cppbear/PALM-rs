// Answer 0

#[test]
fn test_deserialize_any_array() {
    use serde::de::{self, Visitor};

    struct TestVisitor {
        values: Vec<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some(value) = visitor.next_element()? {
                result.push(value);
            }
            Ok(result)
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::custom("Expected an array, found a boolean"))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::custom("Expected an array, found null"))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(E::custom("Expected an array, found a string"))
        }
        // Implement other necessary visit methods if required
    }

    let array_value = Value::Array(vec![
        Value::Number(Number { n: 1 }), 
        Value::Number(Number { n: 2 }), 
        Value::Number(Number { n: 3 })
    ]);

    let visitor = TestVisitor { values: Vec::new() };
    let result = array_value.deserialize_any(visitor);
    assert!(result.is_ok());
    let result_values = result.unwrap();
    assert_eq!(result_values, vec![1, 2, 3]);
}

#[test]
fn test_deserialize_any_nil_array() {
    use serde::de::{self, Visitor};

    struct TestVisitor {
        values: Vec<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(Vec::new())
        }
    }

    let empty_array_value = Value::Array(Vec::new());

    let visitor = TestVisitor { values: Vec::new() };
    let result = empty_array_value.deserialize_any(visitor);
    assert!(result.is_ok());
    let result_values = result.unwrap();
    assert!(result_values.is_empty());
}

