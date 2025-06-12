// Answer 0

#[test]
fn test_visit_map_with_duplicate_start() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<Idx>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                self.current += 1;
                Ok(Some(self.keys[self.current - 1].clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            if self.values.is_empty() {
                Err(serde::de::Error::custom("No value present"))
            } else {
                let value = self.values.remove(0);
                Ok(value as V)
            }
        }
    }

    let keys = vec![Field::Start, Field::Start];
    let values = vec![1];
    let mut map = MockMapAccess { keys, values, current: 0 };

    let result: Result<(Idx, Idx), _> = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_with_missing_start() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<Idx>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                self.current += 1;
                Ok(Some(self.keys[self.current - 1].clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Ok(1 as V) // Assuming this will be a valid Idx
        }
    }

    let keys = vec![Field::End];
    let values = vec![];
    let mut map = MockMapAccess { keys, values, current: 0 };

    let result: Result<(Idx, Idx), _> = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_with_duplicate_end() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<Idx>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                self.current += 1;
                Ok(Some(self.keys[self.current - 1].clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Ok(1 as V) // Assuming this will be a valid Idx
        }
    }

    let keys = vec![Field::Start, Field::End, Field::End];
    let values = vec![1];
    let mut map = MockMapAccess { keys, values, current: 0 };

    let result: Result<(Idx, Idx), _> = visit_map(map);
    assert!(result.is_err());
}

