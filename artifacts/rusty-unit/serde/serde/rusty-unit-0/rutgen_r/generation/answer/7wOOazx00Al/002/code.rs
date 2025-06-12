// Answer 0

#[derive(Debug)]
struct MockMapAccess {
    keys: Vec<Field>,
    values: Vec<Idx>,
    current_index: usize,
}

impl<'de> MapAccess<'de> for MockMapAccess {
    type Error = serde::de::value::Error;

    fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
        if self.current_index < self.keys.len() {
            let key = self.keys[self.current_index];
            self.current_index += 1;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value<T>(&mut self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>,
    {
        if let Some(value) = self.values.get(self.current_index - 1) {
            Ok(serde::de::Deserialize::deserialize(*value).unwrap())
        } else {
            Err(self::de::value::Error::custom("Value not found"))
        }
    }
}

#[test]
fn test_visit_map_with_valid_input() {
    let keys = vec![Field::End];
    let values = vec![Idx(42)];
    let mut map_access = MockMapAccess {
        keys,
        values,
        current_index: 0,
    };
    
    let result = visit_map(map_access);
    assert_eq!(result, Ok(Idx(42)));
}

#[test]
#[should_panic(expected = "missing field")]
fn test_visit_map_without_end_field() {
    let keys = vec![];
    let values = vec![];
    let mut map_access = MockMapAccess {
        keys,
        values,
        current_index: 0,
    };
    
    let result = visit_map(map_access);
    // This test should panic due to missing the "end" field.
}

#[test]
#[should_panic(expected = "duplicate field")]
fn test_visit_map_with_duplicate_end_field() {
    let keys = vec![Field::End, Field::End];
    let values = vec![Idx(42), Idx(100)];
    let mut map_access = MockMapAccess {
        keys,
        values,
        current_index: 0,
    };
    
    let result = visit_map(map_access);
    // This test should panic due to duplicate "end" field.
}

