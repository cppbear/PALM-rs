// Answer 0

#[derive(Debug)]
struct MockMapAccess {
    keys: Vec<Field>,
    values: Vec<Idx>,
    index: usize,
}

impl<'de> MapAccess<'de> for MockMapAccess {
    type Error = serde::de::value::Error;

    fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
        if self.index < self.keys.len() {
            let key = self.keys[self.index].clone();
            self.index += 1;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value<T>(&mut self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>,
    {
        let value = self.values[self.index - 1].clone();
        Ok(value as T)
    }
}

#[test]
fn test_visit_map_with_valid_end() {
    let keys = vec![Field::End];
    let values: Vec<Idx> = vec![42];
    let mut map_access = MockMapAccess { keys, values, index: 0 };

    let result: Result<Idx, _> = visit_map(map_access);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_visit_map_with_missing_end() {
    let keys = vec![];
    let values: Vec<Idx> = vec![];
    let mut map_access = MockMapAccess { keys, values, index: 0 };

    let result: Result<Idx, _> = visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_with_duplicate_end() {
    let keys = vec![Field::End, Field::End];
    let values: Vec<Idx> = vec![42, 100];
    let mut map_access = MockMapAccess { keys, values, index: 0 };

    let result: Result<Idx, _> = visit_map(map_access);
    assert!(result.is_err());
}

