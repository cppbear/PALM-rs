// Answer 0

#[test]
fn test_visit_map_duplicate_end_field() {
    use std::collections::HashMap;

    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = Box<dyn std::error::Error>;

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
            if self.index <= self.values.len() {
                let value: T = serde_json::from_value(serde_json::json!(self.values[self.index - 1]))?;
                Ok(value)
            } else {
                Err("Not enough values".into())
            }
        }
    }

    let keys = vec![Field::Start, Field::End, Field::End];
    let values = vec![1, 1, 1];

    let mut map_access = MockMapAccess { keys, values, index: 0 };
    let visitor = RangeVisitor::<i32> { expecting: "a range", phantom: std::marker::PhantomData };
    
    let _ = visitor.visit_map(&mut map_access);
}

