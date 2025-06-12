// Answer 0

#[derive(Debug)]
struct TestMapAccess {
    keys: Vec<Field>,
    values: Vec<Idx>,
    index: usize,
}

impl TestMapAccess {
    fn new(keys: Vec<Field>, values: Vec<Idx>) -> Self {
        Self { keys, values, index: 0 }
    }
}

impl<'de> MapAccess<'de> for TestMapAccess {
    type Error = serde::de::value::Error;

    fn next_key(&mut self) -> Result<Option<Self::Key>, Self::Error> {
        if self.index >= self.keys.len() {
            return Ok(None);
        }
        let key = self.keys[self.index];
        self.index += 1;
        Ok(Some(key))
    }

    fn next_value(&mut self) -> Result<Self::Value, Self::Error> {
        if self.index > self.values.len() {
            return Err(serde::de::value::Error::custom("out of bounds for values"));
        }
        let value = self.values[self.index - 1];
        Ok(value)
    }
}

#[test]
fn test_visit_map_success() {
    let map = TestMapAccess::new(vec![Field::Start, Field::End], vec![Idx(1), Idx(2)]);
    let result: Result<(Idx, Idx), _> = visit_map(map);
    assert_eq!(result, Ok((Idx(1), Idx(2))));
}

#[test]
fn test_visit_map_missing_start() {
    let map = TestMapAccess::new(vec![Field::End], vec![Idx(2)]);
    let result: Result<(Idx, Idx), _> = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_missing_end() {
    let map = TestMapAccess::new(vec![Field::Start], vec![Idx(1)]);
    let result: Result<(Idx, Idx), _> = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_duplicate_start() {
    let map = TestMapAccess::new(vec![Field::Start, Field::Start, Field::End], vec![Idx(1), Idx(3), Idx(2)]);
    let result: Result<(Idx, Idx), _> = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_duplicate_end() {
    let map = TestMapAccess::new(vec![Field::Start, Field::End, Field::End], vec![Idx(1), Idx(2), Idx(4)]);
    let result: Result<(Idx, Idx), _> = visit_map(map);
    assert!(result.is_err());
}

