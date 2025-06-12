// Answer 0

#[test]
fn test_visit_map_duplicate_field_start() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
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
            if self.index - 1 < self.values.len() {
                let value: V = bincode::deserialize(&self.values[self.index - 1].to_le_bytes()).unwrap();
                Ok(value)
            } else {
                Err(serde::de::Error::custom("No more values"))
            }
        }
    }

    let keys = vec![Field::Start, Field::Start];
    let values = vec![1, 1];
    let map_access = MockMapAccess { keys, values, index: 0 };
    
    let visitor = RangeFromVisitor { expecting: "Expecting range from visitor", phantom: std::marker::PhantomData::<i32> };
    
    let result = visitor.visit_map(map_access);
}

