// Answer 0

#[test]
fn test_visit_map_duplicate_start() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
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
            let value = self.values[self.index - 1];
            Ok(value as V)
        }
    }

    let keys = vec![Field::Start, Field::Start]; // Duplicate field
    let values = vec![1, 2]; // Values for the keys
    let mut map = MockMap { keys, values, index: 0 };

    let visitor = RangeVisitor::<i32> { expecting: "expected a range", phantom: std::marker::PhantomData };
    
    let _ = visitor.visit_map(&mut map);
}

#[test]
fn test_visit_map_missing_start() {
    struct MockMap {
        keys: Vec<Field>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
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
            Err(serde::de::value::Error::custom("unexpected value"))
        }
    }

    let keys = vec![Field::End]; // Missing `Field::Start`
    let mut map = MockMap { keys, index: 0 };

    let visitor = RangeVisitor::<i32> { expecting: "expected a range", phantom: std::marker::PhantomData };
    
    let _ = visitor.visit_map(&mut map);
}

#[test]
fn test_visit_map_missing_end() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
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
            let value = self.values[self.index - 1];
            Ok(value as V)
        }
    }

    let keys = vec![Field::Start]; // Missing `Field::End`
    let values = vec![1]; // Single value for the keys
    let mut map = MockMap { keys, values, index: 0 };

    let visitor = RangeVisitor::<i32> { expecting: "expected a range", phantom: std::marker::PhantomData };
    
    let _ = visitor.visit_map(&mut map);
}

