// Answer 0

#[test]
fn test_visit_map_missing_end() {
    struct FakeMapAccess {
        next_key_result: Vec<Option<Field>>,
        next_value_result: Vec<Result<Idx, ()>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for FakeMapAccess {
        type Error = ();

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.next_key_result.len() {
                let result = self.next_key_result[self.index];
                self.index += 1;
                Ok(result)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            if let Some(Ok(value)) = self.next_value_result.get(self.index - 1) {
                Ok(value.clone() as V)
            } else {
                Err(())
            }
        }
    }

    let fake_map = FakeMapAccess {
        next_key_result: vec![Some(Field::End)], // Only the Field::End is present, no valid end field is encountered
        next_value_result: vec![Ok(Idx::default())], // Provide a default for the next value call
        index: 0,
    };

    let result: Result<Idx, ()> = visit_map(fake_map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_duplicate_end() {
    struct DuplicateEndMapAccess {
        keys: Vec<Option<Field>>,
        values: Vec<Idx>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for DuplicateEndMapAccess {
        type Error = ();

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let result = self.keys[self.index];
                self.index += 1;
                Ok(result)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            if self.index <= self.values.len() {
                Ok(self.values[self.index - 1] as V)
            } else {
                Err(())
            }
        }
    }

    let duplicate_map = DuplicateEndMapAccess {
        keys: vec![Some(Field::End), Some(Field::End)], // Triggering a duplicate field condition
        values: vec![Idx::default(), Idx::default()],
        index: 0,
    };

    let result: Result<Idx, ()> = visit_map(duplicate_map);
    assert!(result.is_err());
}

