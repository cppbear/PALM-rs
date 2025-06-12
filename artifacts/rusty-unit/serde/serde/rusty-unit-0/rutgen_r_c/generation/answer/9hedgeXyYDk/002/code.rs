// Answer 0

#[test]
fn test_visit_map_valid_case() {
    use crate::de::{MapAccess, Error, Deserialize};
    use std::marker::PhantomData;

    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = Some(self.keys[self.index]);
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index <= self.values.len() {
                let value: T = serde_json::from_value(serde_json::json!(self.values[self.index - 1])).unwrap();
                Ok(value)
            } else {
                Err(Error::custom("No more values"))
            }
        }
    }

    let keys = vec![Field::Start];
    let values = vec![42];
    let mut map_access = MockMapAccess { keys, values, index: 0 };
    let visitor = RangeFromVisitor {
        expecting: "an integer",
        phantom: PhantomData,
    };

    let result: Result<i32, _> = visitor.visit_map(&mut map_access);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_visit_map_missing_field() {
    use crate::de::{MapAccess, Error, Deserialize};
    use std::marker::PhantomData;

    struct MockMapAccess {
        keys: Vec<Field>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = Some(self.keys[self.index]);
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Err(Error::custom("No more values"))
        }
    }

    let keys: Vec<Field> = vec![];
    let mut map_access = MockMapAccess { keys, index: 0 };
    let visitor = RangeFromVisitor {
        expecting: "an integer",
        phantom: PhantomData,
    };

    let result: Result<i32, _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_duplicate_field() {
    use crate::de::{MapAccess, Error, Deserialize};
    use std::marker::PhantomData;

    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = Some(self.keys[self.index]);
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index <= self.values.len() {
                let value: T = serde_json::from_value(serde_json::json!(self.values[self.index - 1])).unwrap();
                Ok(value)
            } else {
                Err(Error::custom("No more values"))
            }
        }
    }

    let keys = vec![Field::Start, Field::Start];
    let values = vec![42, 43];
    let mut map_access = MockMapAccess { keys, values, index: 0 };
    let visitor = RangeFromVisitor {
        expecting: "an integer",
        phantom: PhantomData,
    };

    let result: Result<i32, _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err());
}

