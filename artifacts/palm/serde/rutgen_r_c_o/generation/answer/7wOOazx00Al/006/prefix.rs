// Answer 0

#[test]
fn test_visit_map_with_duplicate_field_error() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = ();

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
                self.index += 1;
                Ok(Some(key))
            } else {
                Err(())
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index <= self.values.len() {
                let value: T = serde::de::from_value(serde_json::json!(self.values[self.index - 1])).unwrap();
                Ok(value)
            } else {
                Err(())
            }
        }
    }

    let mut map = MockMap {
        keys: vec![Field::End, Field::End],
        values: vec![1],
        index: 0,
    };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: PhantomData,
    };

    let _ = visitor.visit_map(&mut map);
}

#[test]
fn test_visit_map_with_missing_field_error() {
    struct MockMap {
        keys: Vec<Field>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = ();

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
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
            Err(())
        }
    }

    let mut map = MockMap {
        keys: vec![Field::End],
        index: 0,
    };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: PhantomData,
    };

    let _ = visitor.visit_map(&mut map);
}

#[test]
fn test_visit_map_with_ok_case() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = ();

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
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
            if self.index <= self.values.len() {
                let value: T = serde::de::from_value(serde_json::json!(self.values[self.index - 1])).unwrap();
                Ok(value)
            } else {
                Err(())
            }
        }
    }

    let mut map = MockMap {
        keys: vec![Field::End],
        values: vec![1],
        index: 0,
    };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: PhantomData,
    };

    let _ = visitor.visit_map(&mut map);
}

