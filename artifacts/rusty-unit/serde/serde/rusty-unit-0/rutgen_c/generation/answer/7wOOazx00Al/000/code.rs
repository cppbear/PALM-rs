// Answer 0

#[test]
fn test_visit_map_valid_case() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
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
            let value = self.values.remove(0);
            Ok(value as T)
        }
    }

    let keys = vec![Field::End];
    let values = vec![42];
    let map_access = MockMapAccess { keys, values, index: 0 };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: std::marker::PhantomData,
    };
    
    let result: Result<i32, _> = visitor.visit_map(map_access);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_visit_map_missing_end_field() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
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
            Err(serde::de::value::Error::custom("Value error"))
        }
    }

    let keys = vec![]; // No keys at all
    let values: Vec<i32> = vec![];
    let map_access = MockMapAccess { keys, values, index: 0 };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: std::marker::PhantomData,
    };

    let result: Result<i32, _> = visitor.visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_duplicate_end_field() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
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
            let value = self.values.remove(0);
            Ok(value as T)
        }
    }

    let keys = vec![Field::End, Field::End]; // Duplicate keys
    let values = vec![42, 43];
    let map_access = MockMapAccess { keys, values, index: 0 };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: std::marker::PhantomData,
    };

    let result: Result<i32, _> = visitor.visit_map(map_access);
    assert!(result.is_err());
}

