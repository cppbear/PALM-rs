// Answer 0

#[test]
fn test_deserialize_tuple_empty() {
    struct DummyVisitor;

    impl<'de> serde::de::Visitor<'de> for DummyVisitor {
        type Value = Vec<usize>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an unsigned integer")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, serde_json::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<usize>()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let input = serde_json::from_str("[]").unwrap();
    let result: Result<Vec<usize>, _> = input.deserialize_tuple(0, DummyVisitor);
    assert_eq!(result.unwrap(), Vec::<usize>::new());
}

#[test]
fn test_deserialize_tuple_single_element() {
    struct DummyVisitor;

    impl<'de> serde::de::Visitor<'de> for DummyVisitor {
        type Value = Vec<usize>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an unsigned integer")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, serde_json::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<usize>()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let input = serde_json::from_str("[1]").unwrap();
    let result: Result<Vec<usize>, _> = input.deserialize_tuple(1, DummyVisitor);
    assert_eq!(result.unwrap(), vec![1]);
}

#[test]
fn test_deserialize_tuple_multiple_elements() {
    struct DummyVisitor;

    impl<'de> serde::de::Visitor<'de> for DummyVisitor {
        type Value = Vec<usize>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an unsigned integer")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, serde_json::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<usize>()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let input = serde_json::from_str("[1, 2, 3]").unwrap();
    let result: Result<Vec<usize>, _> = input.deserialize_tuple(3, DummyVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic(expected = "expected sequence of length 2 or more")]
fn test_deserialize_tuple_insufficient_elements() {
    struct DummyVisitor;

    impl<'de> serde::de::Visitor<'de> for DummyVisitor {
        type Value = Vec<usize>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an unsigned integer")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, serde_json::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<usize>()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let input = serde_json::from_str("[1]").unwrap();
    let _result: Result<Vec<usize>, _> = input.deserialize_tuple(2, DummyVisitor);
}

