// Answer 0

#[test]
fn test_visit_map_valid_case() {
    struct MockMap {
        values: Vec<(Field, i32)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.values.len() {
                let key = self.values[self.index].0;
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value(&mut self) -> Result<i32, Self::Error> {
            if self.index > 0 {
                Ok(self.values[self.index - 1].1)
            } else {
                Err(serde::de::Error::missing_field("value"))
            }
        }
    }

    let values = vec![(Field::End, 10)];
    let mock_map = MockMap { values, index: 0 };
    let visitor = RangeToVisitor { expecting: "an integer", phantom: std::marker::PhantomData::<i32> };
    
    let _ = visitor.visit_map(mock_map);
}

#[test]
fn test_visit_map_duplicate_field() {
    struct MockMap {
        values: Vec<(Field, i32)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.values.len() {
                let key = self.values[self.index].0;
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value(&mut self) -> Result<i32, Self::Error> {
            if self.index > 0 {
                Ok(self.values[self.index - 1].1)
            } else {
                Err(serde::de::Error::missing_field("value"))
            }
        }
    }

    let values = vec![(Field::End, 10), (Field::End, 20)];
    let mock_map = MockMap { values, index: 0 };
    let visitor = RangeToVisitor { expecting: "an integer", phantom: std::marker::PhantomData::<i32> };
    
    let result = visitor.visit_map(mock_map);
    let _ = result.is_err(); // Expect an error for duplicate field
}

#[test]
fn test_visit_map_missing_field() {
    struct MockMap {
        values: Vec<(Field, i32)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.values.len() {
                let key = self.values[self.index].0;
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value(&mut self) -> Result<i32, Self::Error> {
            Err(serde::de::Error::missing_field("value"))
        }
    }

    let values: Vec<(Field, i32)> = vec![]; // No Field::End present
    let mock_map = MockMap { values, index: 0 };
    let visitor = RangeToVisitor { expecting: "an integer", phantom: std::marker::PhantomData::<i32> };

    let result = visitor.visit_map(mock_map);
    let _ = result.is_err(); // Expect an error for missing field
}

