// Answer 0

#[test]
fn test_visit_map_with_missing_field() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::ValueAccessError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
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
            if self.index <= self.values.len() {
                let value = self.values[self.index - 1];
                Ok(value as V)
            } else {
                Err(serde::de::ValueAccessError::custom("No value available"))
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![Field::End],
        values: vec![],
        index: 0,
    };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: std::marker::PhantomData,
    };

    let result: Result<i32, _> = visitor.visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_with_duplicate_field() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::ValueAccessError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
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
            if self.index <= self.values.len() {
                let value = self.values[self.index - 1];
                Ok(value as V)
            } else {
                Err(serde::de::ValueAccessError::custom("No value available"))
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![Field::End, Field::End],
        values: vec![42],
        index: 0,
    };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: std::marker::PhantomData,
    };

    let result: Result<i32, _> = visitor.visit_map(map_access);
    assert!(result.is_err());
} 

#[test]
fn test_visit_map_with_no_end_field() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::ValueAccessError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
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
            if self.index <= self.values.len() {
                let value = self.values[self.index - 1];
                Ok(value as V)
            } else {
                Err(serde::de::ValueAccessError::custom("No value available"))
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![], // No keys, simulating absence of "end"
        values: vec![],
        index: 0,
    };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: std::marker::PhantomData,
    };

    let result: Result<i32, _> = visitor.visit_map(map_access);
    assert!(result.is_err());
}

