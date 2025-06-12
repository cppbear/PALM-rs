// Answer 0

#[test]
fn test_deserialize_seq_with_valid_array() {
    // Arrange
    let value = serde_json::Value::Array(vec![
        serde_json::Value::Number(serde_json::Number::from(1)),
        serde_json::Value::Number(serde_json::Number::from(2)),
        serde_json::Value::Number(serde_json::Number::from(3)),
    ]);

    struct TestVisitor {
        values: Vec<i64>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i64>;

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element::<serde_json::Value>()? {
                if let serde_json::Value::Number(num) = value {
                    if let Some(n) = num.as_i64() {
                        values.push(n);
                    }
                }
            }
            Ok(values)
        }

        serde::forward_to_deserialize_any! {
            bytes byte_buf char map newtype_struct none seq string struct tuple tuple_struct
            unit unit_struct
        }
    }

    // Act
    let result: Result<Vec<i64>, serde_json::Error> = value.deserialize_seq(TestVisitor { values: Vec::new() });

    // Assert
    assert!(result.is_ok());
    let values = result.unwrap();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_deserialize_seq_with_invalid_type() {
    // Arrange
    let value = serde_json::Value::Bool(true);

    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("should not be called"))
        }

        serde::forward_to_deserialize_any! {
            bytes byte_buf char map newtype_struct none seq string struct tuple tuple_struct
            unit unit_struct
        }
    }

    // Act
    let result: Result<(), serde_json::Error> = value.deserialize_seq(TestVisitor);

    // Assert
    assert!(result.is_err());
}

#[test]
fn test_deserialize_seq_with_empty_array() {
    // Arrange
    let value = serde_json::Value::Array(vec![]);

    struct TestVisitor {
        count: usize,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = usize;

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut count = 0;
            while seq.next_element::<serde_json::Value>()?.is_some() {
                count += 1;
            }
            Ok(count)
        }

        serde::forward_to_deserialize_any! {
            bytes byte_buf char map newtype_struct none seq string struct tuple tuple_struct
            unit unit_struct
        }
    }

    // Act
    let result: Result<usize, serde_json::Error> = value.deserialize_seq(TestVisitor { count: 0 });

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

