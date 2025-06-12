// Answer 0

#[derive(Debug)]
struct MyMapAccess {
    keys: Vec<Field>,
    values: Vec<Idx>,
    index: usize,
}

impl MyMapAccess {
    fn new(keys: Vec<Field>, values: Vec<Idx>) -> Self {
        Self { keys, values, index: 0 }
    }
}

impl<'de> MapAccess<'de> for MyMapAccess {
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

    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        let value = self.values[self.index - 1].clone(); 
        Ok(value) // Note: Type conversion to V must be handled in real implementation.
    }
}

#[test]
fn test_visit_map_with_valid_data() {
    let keys = vec![Field::Start];
    let values = vec![Idx(1)];
    let map_access = MyMapAccess::new(keys, values);

    let result = visit_map(map_access);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Idx(1));
}

#[test]
fn test_visit_map_with_missing_field() {
    let keys: Vec<Field> = vec![];
    let values: Vec<Idx> = vec![];
    let map_access = MyMapAccess::new(keys, values);

    let result = visit_map(map_access);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().classification(), ErrorKind::MissingField("start"));
}

#[test]
fn test_visit_map_with_duplicate_field() {
    let keys = vec![Field::Start, Field::Start];
    let values = vec![Idx(1), Idx(2)];
    let map_access = MyMapAccess::new(keys, values);

    let result = visit_map(map_access);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().classification(), ErrorKind::DuplicateField("start"));
}

