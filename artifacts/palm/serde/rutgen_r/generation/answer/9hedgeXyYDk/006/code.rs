// Answer 0

#[derive(Debug)]
struct MockMapAccess {
    keys: Vec<Field>,
    values: Vec<Idx>,
    index: usize,
}

impl<'de> MapAccess<'de> for MockMapAccess {
    type Error = &'static str;

    fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
        if self.index < self.keys.len() {
            let key = self.keys[self.index];
            self.index += 1;
            Ok(Some(key))
        } else {
            Err("No more keys")
        }
    }

    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        if self.index <= self.values.len() {
            let value = self.values[self.index - 1];
            Ok(value as V)
        } else {
            Err("No more values")
        }
    }
}

#[derive(Debug)]
struct Idx(u32);

#[derive(Debug)]
enum Field {
    Start,
}

#[test]
fn test_visit_map_with_duplicate_start_key() {
    let mut map = MockMapAccess {
        keys: vec![Field::Start, Field::Start],
        values: vec![Idx(1), Idx(2)],
        index: 0,
    };

    let result = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_with_missing_start_key() {
    let mut map = MockMapAccess {
        keys: vec![],
        values: vec![],
        index: 0,
    };

    let result = visit_map(map);
    assert!(result.is_err());
}

