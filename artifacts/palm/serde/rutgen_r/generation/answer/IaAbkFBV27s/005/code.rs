// Answer 0

#[derive(Debug)]
struct DummyMapAccess {
    keys: Vec<Field>,
    values: Vec<Idx>,
    current_index: usize,
}

impl<'de> MapAccess<'de> for DummyMapAccess {
    type Error = serde::de::value::Error; // Assuming this as the error type
    
    fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
        if self.current_index < self.keys.len() {
            let key = self.keys[self.current_index].clone();
            self.current_index += 1;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        if self.current_index - 1 < self.values.len() {
            let value = self.values[self.current_index - 1]; // return the value corresponding to the current key
            // Assume V can be constructed from Idx for this test
            Ok(value as V) 
        } else {
            Err(Self::Error::custom("Missing value"))
        }
    }
}

#[test]
fn test_visit_map_duplicate_end_field() {
    let keys = vec![Field::Start, Field::End, Field::End]; // Duplicate "end" field
    let values = vec![1, 2]; // Possible values
    let map_access = DummyMapAccess {
        keys,
        values,
        current_index: 0,
    };
    
    let result: Result<(Idx, Idx), <DummyMapAccess as MapAccess<'_>>::Error> = visit_map(map_access);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.to_string(), "duplicate field `end`");
    }
}

