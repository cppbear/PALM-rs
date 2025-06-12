// Answer 0

#[derive(Debug)]
struct MockMapAccess {
    keys: Vec<Field>,
    values: Vec<Idx>,
    current: usize,
}

impl<'de> MapAccess<'de> for MockMapAccess {
    type Error = MockError;

    fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
        if self.current < self.keys.len() {
            let key = self.keys[self.current].clone();
            self.current += 1;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value<T>(&mut self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>,
    {
        if self.current <= self.values.len() {
            let value = self.values[self.current - 1].clone();
            // Placeholder for deserialization logic
            Ok(value as T) 
        } else {
            Err(MockError)
        }
    }
}

#[derive(Debug)]
struct MockError;

impl Error for MockError {
    fn duplicate_field(_: &str) -> Self {
        MockError
    }

    fn missing_field(_: &str) -> Self {
        MockError
    }
}

#[test]
fn test_visit_map_duplicate_end_field() {
    let mut map = MockMapAccess {
        keys: vec![Field::End, Field::End],
        values: vec![Idx(1), Idx(2)], // Simulates two "end" fields with different values
        current: 0,
    };

    let result: Result<Idx, MockError> = visit_map(map);
    assert!(result.is_err(), "Expected an error for duplicate 'end' field");
}

#[derive(Clone)]
struct Idx(u32);

#[derive(Clone)]
enum Field {
    End,
}

trait MapAccess<'de> {
    type Error;

    fn next_key(&mut self) -> Result<Option<Field>, Self::Error>;
    fn next_value<T>(&mut self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>;
} 

trait Error {
    fn duplicate_field(_: &str) -> Self;
    fn missing_field(_: &str) -> Self;
} 

trait Deserialize<'de> {} 

impl Deserialize<'static> for Idx {}

