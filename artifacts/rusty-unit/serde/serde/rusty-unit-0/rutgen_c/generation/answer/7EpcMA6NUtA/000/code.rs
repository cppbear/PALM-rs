// Answer 0

#[test]
fn test_next_element_seed_with_some_value() {
    struct TestSeed;
    
    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = i32;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_i32(TestVisitor)
        }
    }

    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i32")
        }

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    let input = vec![1, 2, 3].into_iter().map(|x| x.into_deserializer());
    let mut deserializer = SeqDeserializer {
        iter: input.fuse(),
        count: 0,
        marker: PhantomData,
    };

    let result = deserializer.next_element_seed(TestSeed).unwrap();
    assert_eq!(result, Some(1));
    assert_eq!(deserializer.count, 1);
}

#[test]
fn test_next_element_seed_with_none_value() {
    struct TestSeed;

    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = i32;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_i32(TestVisitor)
        }
    }

    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i32")
        }

        fn visit_i32<E>(self, _value: i32) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            // This function will not be called in this test
            Err(E::custom("should not deserialize"))
        }
    }

    let input: Vec<i32> = vec![].into_iter().map(|_| i32::default().into_deserializer()).collect();
    let mut deserializer = SeqDeserializer {
        iter: input.fuse(),
        count: 0,
        marker: PhantomData,
    };

    let result = deserializer.next_element_seed(TestSeed).unwrap();
    assert_eq!(result, None);
    assert_eq!(deserializer.count, 0);
}

