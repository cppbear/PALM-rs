// Answer 0

#[test]
fn test_visit_map_with_duplicate_field_end() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<Option<i32>>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current].clone();
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
                if let Some(value) = self.values.get(self.current - 1) {
                    if let Some(v) = value {
                        return Ok(*v as V);
                    }
                }
            }
            Err(serde::de::Error::custom("No value found"))
        }
    }

    let mut map_access = MockMapAccess {
        keys: vec![Field::End, Field::End], // duplicate "end" keys
        values: vec![Some(42), Some(43)],
        current: 0,
    };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an i32 value",
        phantom: std::marker::PhantomData,
    };

    let _ = visitor.visit_map(&mut map_access);
}

