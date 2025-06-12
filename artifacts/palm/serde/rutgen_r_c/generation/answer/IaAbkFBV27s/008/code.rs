// Answer 0

fn test_visit_map_duplicate_start() {
    struct FakeMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for FakeMapAccess {
        type Error = &'static str;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                self.index += 1;
                Ok(Some(self.keys[self.index - 1]))
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index > 0 && self.index - 1 < self.values.len() {
                let value = self.values[self.index - 1];
                self.index += 1;
                Ok(value as T) // Unsafe casting for test purposes
            } else {
                Err("No more values")
            }
        }
    }

    let keys = vec![Field::Start, Field::Start];
    let values = vec![1, 2];
    let mut map_access = FakeMapAccess { keys, values, index: 0 };
    let visitor = RangeVisitor { expecting: "a range", phantom: PhantomData::<i32> };

    let result: Result<(i32, i32), _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("duplicate field: start"));
}

fn test_visit_map_missing_start() {
    struct FakeMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for FakeMapAccess {
        type Error = &'static str;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                self.index += 1;
                Ok(Some(self.keys[self.index - 1]))
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index > 0 && self.index - 1 < self.values.len() {
                let value = self.values[self.index - 1];
                self.index += 1;
                Ok(value as T) // Unsafe casting for test purposes
            } else {
                Err("No more values")
            }
        }
    }

    let keys = vec![Field::End];
    let values = vec![2];
    let mut map_access = FakeMapAccess { keys, values, index: 0 };
    let visitor = RangeVisitor { expecting: "a range", phantom: PhantomData::<i32> };

    let result: Result<(i32, i32), _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("missing field: start"));
}

fn test_visit_map_missing_end() {
    struct FakeMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for FakeMapAccess {
        type Error = &'static str;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                self.index += 1;
                Ok(Some(self.keys[self.index - 1]))
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index > 0 && self.index - 1 < self.values.len() {
                let value = self.values[self.index - 1];
                self.index += 1;
                Ok(value as T) // Unsafe casting for test purposes
            } else {
                Err("No more values")
            }
        }
    }

    let keys = vec![Field::Start];
    let values = vec![1];
    let mut map_access = FakeMapAccess { keys, values, index: 0 };
    let visitor = RangeVisitor { expecting: "a range", phantom: PhantomData::<i32> };

    let result: Result<(i32, i32), _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err());
    assert_eq!(result.err(), Some("missing field: end"));
}

