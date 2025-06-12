// Answer 0

#[derive(Debug)]
struct MockIter {
    values: Vec<&'static str>,
    index: usize,
}

impl MockIter {
    fn new(values: Vec<&'static str>) -> Self {
        Self { values, index: 0 }
    }
}

impl Iterator for MockIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len() {
            let value = self.values[self.index];
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct MockSeed;

impl<'de> DeserializeSeed<'de> for MockSeed {
    type Value = String;

    fn deserialize<T>(self, value: T) -> Result<Self::Value, Error>
    where
        T: serde::de::Deserialize<'de>,
    {
        value.deserialize(serde_json::Deserializer::from_str(value)).map_err(|_| Error::custom("Deserialization error"))
    }
}

#[test]
fn test_next_element_seed_some_value() {
    let mut mock_iter = MockIter::new(vec!["\"value1\"", "\"value2\""]);
    let seed = MockSeed;

    let result_1 = mock_iter.next_element_seed(seed);
    assert_eq!(result_1.unwrap(), Some("value1".to_string()));

    let result_2 = mock_iter.next_element_seed(seed);
    assert_eq!(result_2.unwrap(), Some("value2".to_string()));
}

#[test]
fn test_next_element_seed_none_value() {
    let mut mock_iter = MockIter::new(vec![]);
    let seed = MockSeed;

    let result = mock_iter.next_element_seed(seed);
    assert_eq!(result.unwrap(), None);
}

