// Answer 0

fn test_next_element_seed_some() {
    struct MockSeed {
        value: Option<Value>,
    }

    impl<'de> DeserializeSeed<'de> for MockSeed {
        type Value = Value;

        fn deserialize<D>(self, _: D) -> Result<Self::Value, Error>
        where
            D: Deserializer<'de>,
        {
            Ok(self.value.unwrap())
        }
    }

    let values = vec![
        Value::Bool(true),
        Value::Number(Number::from(42)),
        Value::String("test".to_owned()),
    ];
    let mut deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    for expected in &values {
        let seed = MockSeed {
            value: Some(expected.clone()),
        };
        let result = deserializer.next_element_seed(seed).unwrap();
        assert_eq!(result, Some(expected.clone()));
    }
}

fn test_next_element_seed_none() {
    struct MockSeed;

    impl<'de> DeserializeSeed<'de> for MockSeed {
        type Value = Value;

        fn deserialize<D>(self, _: D) -> Result<Self::Value, Error>
        where
            D: Deserializer<'de>,
        {
            Err(Error) // Just a mock to satisfy the trait
        }
    }

    let values: Vec<Value> = vec![];
    let mut deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    let seed = MockSeed;
    let result = deserializer.next_element_seed(seed).unwrap();
    assert_eq!(result, None);
}

