// Answer 0

#[derive(Debug)]
struct TestError;

impl serde::de::Error for TestError {
    fn custom<T>(_msg: T) -> Self {
        TestError
    }
    
    fn duplicate_field(_field: &'static str) -> Self {
        TestError
    }

    fn missing_field(_field: &'static str) -> Self {
        TestError
    }
}

struct TestMapAccess {
    keys: Vec<Field>,
    values: Vec<Idx>,
    current_index: usize,
}

impl<'de> serde::de::MapAccess<'de> for TestMapAccess {
    type Error = TestError;

    fn next_key<T>(&mut self) -> Result<Option<Field>, Self::Error> {
        if self.current_index < self.keys.len() {
            let key = self.keys[self.current_index];
            self.current_index += 1;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value<T>(&mut self) -> Result<Idx, Self::Error> {
        if self.current_index > 0 && self.current_index-1 < self.values.len() {
            Ok(self.values[self.current_index - 1])
        } else {
            Err(TestError)
        }
    }
}

#[test]
fn test_visit_map_error_missing_start() {
    let map_access = TestMapAccess {
        keys: vec![Field::End],
        values: vec![Idx(10)],
        current_index: 0,
    };
    let result: Result<(Idx, Idx), TestError> = visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_error_duplicate_start() {
    let map_access = TestMapAccess {
        keys: vec![Field::Start, Field::Start],
        values: vec![Idx(1), Idx(2)],
        current_index: 0,
    };
    let result: Result<(Idx, Idx), TestError> = visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_error_duplicate_end() {
    let map_access = TestMapAccess {
        keys: vec![Field::End, Field::End],
        values: vec![Idx(1), Idx(2)],
        current_index: 0,
    };
    let result: Result<(Idx, Idx), TestError> = visit_map(map_access);
    assert!(result.is_err());
}

