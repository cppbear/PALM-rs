// Answer 0

#[test]
fn test_next_element_seed_none() {
    struct TestIter<'de> {
        iter: &'de [&'de str],
        count: usize,
    }

    impl<'de> TestIter<'de> {
        fn new(iter: &'de [&'de str]) -> Self {
            TestIter { iter, count: 0 }
        }

        fn next(&mut self) -> Option<&'de str> {
            if self.count < self.iter.len() {
                let item = self.iter[self.count];
                self.count += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    struct TestDeserializer;

    impl<'de> de::DeserializeSeed<'de> for TestDeserializer {
        type Value = String;

        fn deserialize<Dem>(&self, _deserializer: Dem) -> Result<Self::Value, Dem::Error>
        where
            Dem: de::Deserializer<'de>,
        {
            Ok("".to_string())
        }
    }

    let mut test_iter = TestIter::new(&[]);
    let seed = TestDeserializer;

    let result: Result<Option<String>, ()> = test_iter.next_element_seed(seed);
    assert_eq!(result, Ok(None));
}

