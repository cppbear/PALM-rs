// Answer 0

fn test_visit_map_missing_end() {
    use serde::de::{MapAccess, Visitor};
    use serde::de::Error;

    struct MockMap {
        keys: Vec<Field>,
        values: Vec<Idx>,
        index: usize,
    }

    impl MapAccess<'_> for MockMap {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index >= self.keys.len() {
                return Ok(None);
            }
            let key = self.keys[self.index];
            self.index += 1;
            Ok(Some(key))
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: serde::de::Deserialize<'_>,
        {
            if self.index == 0 || self.index > self.values.len() {
                return Err(Self::Error::custom("missing value"));
            }
            let value: T = serde::de::Deserialize::deserialize(serde::de::value::Deserializer::new(self.values[self.index - 1])).unwrap();
            Ok(value)
        }
    }

    let keys = vec![Field::Start]; // Missing Field::End to trigger the error.
    let values = vec![Idx(1)];
    let mock_map = MockMap { keys, values, index: 0 };

    let result: Result<(Idx, Idx), <MockMap as MapAccess<'_>>::Error> = visit_map(mock_map);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "missing field `end`");
}

