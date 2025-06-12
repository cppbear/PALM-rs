// Answer 0

#[test]
fn test_visit_map_duplicate_field() {
    use std::marker::PhantomData;

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

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(serde::de::value::Error::custom("Unexpected error"))
        }
    }

    let visitor = RangeToVisitor::<i32> {
        expecting: "testing duplicate field error",
        phantom: PhantomData,
    };

    let mut map_access = MockMapAccess {
        keys: vec![Field::End, Field::End], // Duplicate field
        values: vec![1, 2],
        index: 0,
    };

    let result: Result<i32, _> = visitor.visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_missing_field() {
    use std::marker::PhantomData;

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

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(serde::de::value::Error::custom("Unexpected error"))
        }
    }

    let visitor = RangeToVisitor::<i32> {
        expecting: "testing missing field error",
        phantom: PhantomData,
    };

    let mut map_access = MockMapAccess {
        keys: vec![], // No keys, "end" field is missing
        values: vec![],
        index: 0,
    };

    let result: Result<i32, _> = visitor.visit_map(map_access);
    assert!(result.is_err());
}

