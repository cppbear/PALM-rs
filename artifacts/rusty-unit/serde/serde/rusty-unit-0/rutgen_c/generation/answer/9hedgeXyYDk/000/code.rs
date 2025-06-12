// Answer 0

#[test]
fn test_visit_map_with_missing_field() {
    struct MockMapAccess<'de> {
        keys: Vec<Option<Field>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess<'de> {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
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
            Err(MockError)
        }
    }

    struct MockVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let visitor = RangeFromVisitor::<i32> {
        expecting: "an i32 value",
        phantom: PhantomData,
    };
    let map_access = MockMapAccess {
        keys: vec![None],
        index: 0,
    };

    let result = visitor.visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_with_duplicate_field() {
    struct MockMapAccess<'de> {
        keys: Vec<Option<Field>>,
        index: usize,
        values: Vec<i32>,
    }

    impl<'de> MapAccess<'de> for MockMapAccess<'de> {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
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
                Ok(self.values.remove(0) as T)
            } else {
                Err(MockError)
            }
        }
    }

    struct MockError;

    impl Error for MockError {}

    let visitor = RangeFromVisitor::<i32> {
        expecting: "an i32 value",
        phantom: PhantomData,
    };
    let map_access = MockMapAccess {
        keys: vec![Some(Field::Start), Some(Field::Start)],
        index: 0,
        values: vec![1],
    };

    let result = visitor.visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_with_valid_input() {
    struct MockMapAccess<'de> {
        keys: Vec<Option<Field>>,
        index: usize,
        values: Vec<i32>,
    }

    impl<'de> MapAccess<'de> for MockMapAccess<'de> {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
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
                Ok(self.values.remove(0) as T)
            } else {
                Err(MockError)
            }
        }
    }

    struct MockError;

    impl Error for MockError {}

    let visitor = RangeFromVisitor::<i32> {
        expecting: "an i32 value",
        phantom: PhantomData,
    };
    let map_access = MockMapAccess {
        keys: vec![Some(Field::Start)],
        index: 0,
        values: vec![42],
    };

    let result = visitor.visit_map(map_access);
    assert_eq!(result, Ok(42));
}

