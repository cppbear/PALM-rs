// Answer 0

#[test]
fn test_deserialize_tuple_empty() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of i32")
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(vec![])
        }
    }

    let input = serde_json::from_str::<serde_json::Value>("[]").unwrap();
    let result = input.deserialize_tuple(0, MockVisitor);
    assert_eq!(result.unwrap(), Vec::<i32>::new());
}

#[test]
fn test_deserialize_tuple_single_element() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of i32")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let value = seq.next_element::<i32>()?.unwrap();
            Ok(vec![value])
        }
    }

    let input = serde_json::from_str::<serde_json::Value>("[1]").unwrap();
    let result = input.deserialize_tuple(1, MockVisitor);
    assert_eq!(result.unwrap(), vec![1]);
}

#[test]
fn test_deserialize_tuple_multiple_elements() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of i32")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let input = serde_json::from_str::<serde_json::Value>("[1, 2, 3]").unwrap();
    let result = input.deserialize_tuple(3, MockVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[should_panic]
#[test]
fn test_deserialize_tuple_incorrect_length() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of i32")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let _value = seq.next_element::<i32>()?; // expects exactly one element
            seq.next_element::<i32>()?; // trying to access a second element will panic
            Ok(vec![])
        }
    }

    let input = serde_json::from_str::<serde_json::Value>("[1]").unwrap();
    let _result = input.deserialize_tuple(2, MockVisitor);
}

