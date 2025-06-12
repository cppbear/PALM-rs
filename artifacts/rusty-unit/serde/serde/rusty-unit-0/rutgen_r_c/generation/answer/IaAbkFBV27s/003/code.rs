// Answer 0

#[test]
fn test_visit_map_missing_end() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl MapAccess<'static> for MockMap {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                self.index += 1;
                Ok(Some(self.keys[self.index - 1].clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'static>,
        {
            if !self.values.is_empty() {
                let value = self.values.remove(0);
                Ok(value as V)
            } else {
                Err(serde::de::Error::custom("No more values"))
            }
        }
    }

    let keys = vec![Field::Start];
    let values = vec![1];  // Only "start" is provided
    let mut map = MockMap { keys, values, index: 0 };

    let visitor = RangeVisitor::<i32> {
        expecting: "an integer range",
        phantom: std::marker::PhantomData,
    };

    let result: Result<(i32, i32), _> = visitor.visit_map(&mut map);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), serde::de::value::Error::missing_field("end"));
}

#[test]
fn test_visit_map_duplicate_start() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl MapAccess<'static> for MockMap {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                self.index += 1;
                Ok(Some(self.keys[self.index - 1].clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'static>,
        {
            if !self.values.is_empty() {
                let value = self.values.remove(0);
                Ok(value as V)
            } else {
                Err(serde::de::Error::custom("No more values"))
            }
        }
    }

    let keys = vec![Field::Start, Field::Start];
    let values = vec![1, 2];  // Duplicate "start"
    let mut map = MockMap { keys, values, index: 0 };

    let visitor = RangeVisitor::<i32> {
        expecting: "an integer range",
        phantom: std::marker::PhantomData,
    };

    let result: Result<(i32, i32), _> = visitor.visit_map(&mut map);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), serde::de::value::Error::duplicate_field("start"));
}

