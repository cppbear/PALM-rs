// Answer 0

#[test]
fn test_visit_map_duplicate_field_error() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<Result<i32, serde::de::Error>>,
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
            V: serde::de::Deserialize<'de>,
        {
            self.values.remove(0).map(|value| value as V)
        }
    }

    let mut map = MockMapAccess {
        keys: vec![Field::End, Field::End],
        values: vec![Err(serde::de::Error::custom("Invalid value")),
                      Err(serde::de::Error::custom("Invalid value"))],
        index: 0,
    };

    let visitor = RangeToVisitor { expecting: "Some value", phantom: PhantomData::<i32> };

    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_missing_field_error() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<Result<i32, serde::de::Error>>,
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
            V: serde::de::Deserialize<'de>,
        {
            self.values.remove(0).map(|value| value as V)
        }
    }

    let mut map = MockMapAccess {
        keys: vec![],
        values: vec![],
        index: 0,
    };

    let visitor = RangeToVisitor { expecting: "Some value", phantom: PhantomData::<i32> };

    let _ = visitor.visit_map(map);
}

