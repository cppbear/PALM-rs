// Answer 0

fn test_visit_map_err_duplicate_field() {
    struct TestMapAccessor {
        keys: Vec<Field>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccessor {
        type Error = &'static str;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = Some(self.keys[self.index]);
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err("value error")
        }
    }

    let mut map = TestMapAccessor {
        keys: vec![Field::Start, Field::Start],
        index: 0,
    };

    let result: Result<Idx, _> = visit_map(map);
    assert!(result.is_err());
}

fn test_visit_map_err_missing_field() {
    struct TestMapAccessor {
        keys: Vec<Field>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccessor {
        type Error = &'static str;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = Some(self.keys[self.index]);
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err("value error")
        }
    }

    let mut map = TestMapAccessor {
        keys: vec![],
        index: 0,
    };

    let result: Result<Idx, _> = visit_map(map);
    assert!(result.is_err());
}

