// Answer 0

#[test]
fn test_visit_map_duplicate_start_field() {
    use serde::de::{MapAccess, Visitor, Deserializer};
    use serde::ser::Serialize;
    use serde::de::{self, Error, Unexpected};
    
    struct DummyMap {
        keys: Vec<Field>,
        values: Vec<Idx>,
        index: usize,
    }

    impl MapAccess<'_> for DummyMap {
        type Error = serde::de::value::Error;

        fn next_key<U>(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            if self.index <= self.values.len() {
                let value = self.values[self.index - 1];
                self.index += 1;
                Ok(value as V)
            } else {
                Err(Self::Error::invalid_value(Unexpected::Other("value"), &"expected a value"))
            }
        }
    }

    let keys = vec![Field::Start, Field::Start]; // Duplicate "start" field
    let values = vec![Idx(1), Idx(2)]; // Sample valid indices for the values
    let map = DummyMap { keys, values, index: 0 };

    let result: Result<(Idx, Idx), <DummyMap as MapAccess<'_>>::Error> = visit_map(map);
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.to_string(), "duplicate field `start`");
    }
}

