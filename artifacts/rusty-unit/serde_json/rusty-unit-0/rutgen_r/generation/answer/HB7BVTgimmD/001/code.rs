// Answer 0

#[test]
fn test_tuple_variant_with_valid_visitor() {
    struct TestVisitor {
        value: Vec<i32>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value>
        where
            S: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    struct TestDeserializer {
        data: Vec<i32>,
    }

    impl TestDeserializer {
        fn new(data: Vec<i32>) -> Self {
            TestDeserializer { data }
        }

        fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            de::Deserializer::deserialize_seq(self.data, visitor)
        }
    }

    let deserializer = TestDeserializer::new(vec![1, 2, 3]);
    let visitor = TestVisitor { value: Vec::new() };
    let result = deserializer.tuple_variant(3, visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_tuple_variant_with_invalid_length() {
    struct PanicVisitor;

    impl<'de> de::Visitor<'de> for PanicVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<S>(self, _seq: S) -> Result<Self::Value>
        where
            S: de::SeqAccess<'de>,
        {
            panic!("Unexpected sequence length");
        }
    }

    struct TestDeserializer;

    impl TestDeserializer {
        fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            Err(serde_json::Error::custom("Invalid input"))
        }
    }

    let deserializer = TestDeserializer {};
    let visitor = PanicVisitor;
    deserializer.tuple_variant(2, visitor).unwrap();
}

#[test]
fn test_tuple_variant_empty_sequence() {
    struct EmptyVisitor;

    impl<'de> de::Visitor<'de> for EmptyVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an empty array of integers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value>
        where
            S: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(_) = seq.next_element::<i32>()? {
                // No-op for empty
            }
            Ok(values)
        }
    }

    struct EmptyDeserializer;

    impl EmptyDeserializer {
        fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            if len == 0 {
                visitor.visit_seq(serde_json::Deserializer::from_slice(&[]))
            } else {
                Err(serde_json::Error::custom("Expected empty sequence"))
            }
        }
    }

    let deserializer = EmptyDeserializer {};
    let result = deserializer.tuple_variant(0, EmptyVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Vec::<i32>::new());
}

