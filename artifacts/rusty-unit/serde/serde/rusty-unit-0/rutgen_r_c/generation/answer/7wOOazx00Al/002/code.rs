// Answer 0

#[test]
fn test_visit_map_valid_case() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error; // Replace with the appropriate error type.

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current];
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            if self.current <= self.values.len() {
                let value = self.values[self.current - 1]; // Use value corresponding to the current index
                // Assume V can be successfully cast to i32 for this test scenario
                Ok(value as V)
            } else {
                Err(serde::de::value::Error::custom("No more values"))
            }
        }
    }

    let visitor = RangeToVisitor::<i32> {
        expecting: "an i32",
        phantom: std::marker::PhantomData,
    };

    let keys = vec![Field::End];
    let values = vec![10]; // Example value

    let mut map_access = MockMapAccess {
        keys,
        values,
        current: 0,
    };

    let result: Result<i32, _> = visitor.visit_map(&mut map_access);
    assert_eq!(result, Ok(10)); // Expect to receive the correct value
}

#[test]
fn test_visit_map_duplicate_keys() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error; // Replace with the appropriate error type.

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current];
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            if self.current <= self.values.len() {
                let value = self.values[self.current - 1];
                Ok(value as V)
            } else {
                Err(serde::de::value::Error::custom("No more values"))
            }
        }
    }

    let visitor = RangeToVisitor::<i32> {
        expecting: "an i32",
        phantom: std::marker::PhantomData,
    };

    let keys = vec![Field::End, Field::End]; // Duplicate key
    let values = vec![10, 20]; // Example value

    let mut map_access = MockMapAccess {
        keys,
        values,
        current: 0,
    };

    let result: Result<i32, _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err()); // Expect an error due to duplicate keys
}

#[test]
fn test_visit_map_missing_key() {
    struct MockMapAccess {
        keys: Vec<Field>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error; // Replace with the appropriate error type.

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current];
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }
        
        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(serde::de::value::Error::custom("No more values"))
        }
    }

    let visitor = RangeToVisitor::<i32> {
        expecting: "an i32",
        phantom: std::marker::PhantomData,
    };

    let keys = vec![]; // No keys
    let values = vec![];

    let mut map_access = MockMapAccess {
        keys,
        current: 0,
    };

    let result: Result<i32, _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err()); // Expect an error due to missing "end" key
}

