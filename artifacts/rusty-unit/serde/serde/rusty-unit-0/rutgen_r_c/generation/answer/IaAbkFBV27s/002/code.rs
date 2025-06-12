// Answer 0

fn test_visit_map_valid() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }
    
    impl MockMap {
        fn new(keys: Vec<Field>, values: Vec<i32>) -> Self {
            MockMap { keys, values, index: 0 }
        }
    }
    
    impl MapAccess<'_> for MockMap {
        type Error = &'static str;

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
            V: Deserialize<'_>,
        {
            if self.index <= self.values.len() {
                Ok(self.values[self.index - 1] as V)
            } else {
                Err("No more values available")
            }
        }
    }

    let visitor = RangeVisitor::<i32> { expecting: "an i32 range", phantom: PhantomData };
    let keys = vec![Field::Start, Field::End];
    let values = vec![1, 10];
    let mock_map = MockMap::new(keys, values);
    
    let result: Result<(i32, i32), _> = visitor.visit_map(mock_map);
    assert_eq!(result, Ok((1, 10)));
}

fn test_visit_map_missing_start() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }
    
    impl MockMap {
        fn new(keys: Vec<Field>, values: Vec<i32>) -> Self {
            MockMap { keys, values, index: 0 }
        }
    }
    
    impl MapAccess<'_> for MockMap {
        type Error = &'static str;

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
            V: Deserialize<'_>,
        {
            if self.index <= self.values.len() {
                Ok(self.values[self.index - 1] as V)
            } else {
                Err("No more values available")
            }
        }
    }

    let visitor = RangeVisitor::<i32> { expecting: "an i32 range", phantom: PhantomData };
    let keys = vec![Field::End];
    let values = vec![10];
    let mock_map = MockMap::new(keys, values);
    
    let result: Result<(i32, i32), _> = visitor.visit_map(mock_map);
    assert!(result.is_err());
}

fn test_visit_map_duplicate_start() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }
    
    impl MockMap {
        fn new(keys: Vec<Field>, values: Vec<i32>) -> Self {
            MockMap { keys, values, index: 0 }
        }
    }
    
    impl MapAccess<'_> for MockMap {
        type Error = &'static str;

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
            V: Deserialize<'_>,
        {
            if self.index <= self.values.len() {
                Ok(self.values[self.index - 1] as V)
            } else {
                Err("No more values available")
            }
        }
    }

    let visitor = RangeVisitor::<i32> { expecting: "an i32 range", phantom: PhantomData };
    let keys = vec![Field::Start, Field::Start];
    let values = vec![1, 2];
    let mock_map = MockMap::new(keys, values);
    
    let result: Result<(i32, i32), _> = visitor.visit_map(mock_map);
    assert!(result.is_err());
}

fn test_visit_map_duplicate_end() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }
    
    impl MockMap {
        fn new(keys: Vec<Field>, values: Vec<i32>) -> Self {
            MockMap { keys, values, index: 0 }
        }
    }
    
    impl MapAccess<'_> for MockMap {
        type Error = &'static str;

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
            V: Deserialize<'_>,
        {
            if self.index <= self.values.len() {
                Ok(self.values[self.index - 1] as V)
            } else {
                Err("No more values available")
            }
        }
    }

    let visitor = RangeVisitor::<i32> { expecting: "an i32 range", phantom: PhantomData };
    let keys = vec![Field::End, Field::End];
    let values = vec![10, 20];
    let mock_map = MockMap::new(keys, values);
    
    let result: Result<(i32, i32), _> = visitor.visit_map(mock_map);
    assert!(result.is_err());
} 

fn test_visit_map_missing_end() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }
    
    impl MockMap {
        fn new(keys: Vec<Field>, values: Vec<i32>) -> Self {
            MockMap { keys, values, index: 0 }
        }
    }
    
    impl MapAccess<'_> for MockMap {
        type Error = &'static str;

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
            V: Deserialize<'_>,
        {
            if self.index <= self.values.len() {
                Ok(self.values[self.index - 1] as V)
            } else {
                Err("No more values available")
            }
        }
    }

    let visitor = RangeVisitor::<i32> { expecting: "an i32 range", phantom: PhantomData };
    let keys = vec![Field::Start];
    let values = vec![1];
    let mock_map = MockMap::new(keys, values);
    
    let result: Result<(i32, i32), _> = visitor.visit_map(mock_map);
    assert!(result.is_err());
}

