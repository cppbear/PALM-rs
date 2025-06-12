// Answer 0

#[derive(Debug)]
struct TestMap {
    keys: Vec<Field>,
    values: Vec<Idx>,
    index: usize,
}

impl TestMap {
    fn new(keys: Vec<Field>, values: Vec<Idx>) -> Self {
        Self { keys, values, index: 0 }
    }
}

impl<'de> MapAccess<'de> for TestMap {
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
        if self.index == 1 {
            self.index += 1; // Simulating a duplicate key
            serde::de::Deserialize::deserialize(&mut serde::de::value::Deserializer::new(self.values[0]))
        } else {
            serde::de::Deserialize::deserialize(&mut serde::de::value::Deserializer::new(self.values[self.index - 1]))
        }
    }
}

#[test]
fn test_visit_map_duplicate_field_error() {
    let keys = vec![Field::Start, Field::Start]; // Two Start keys to trigger duplicate error
    let values = vec![Idx(1), Idx(2)];
    let map = TestMap::new(keys, values);
    
    let result = visit_map(map);
    
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "duplicate field `start`");
}

