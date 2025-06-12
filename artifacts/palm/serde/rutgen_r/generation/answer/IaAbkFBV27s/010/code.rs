// Answer 0

#[test]
fn test_visit_map_duplicate_start() {
    struct TestMap {
        keys: Vec<Field>,
        values: Vec<Idx>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = serde::de::value::Error;
        
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
            T: serde::Deserialize<'de>,
        {
            if self.index <= self.values.len() {
                let value = self.values[self.index - 1];
                self.index += 1;
                Ok(value as T)
            } else {
                Err(serde::de::value::Error::custom("No more values"))
            }
        }
    }

    let mut map = TestMap {
        keys: vec![Field::Start, Field::Start], // Duplicate key
        values: vec![1, 2],
        index: 0,
    };

    let result: Result<(Idx, Idx), serde::de::value::Error> = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_missing_start() {
    struct TestMap {
        keys: Vec<Field>,
        values: Vec<Idx>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = serde::de::value::Error;

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
            T: serde::Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Ok(value as T)
            } else {
                Err(serde::de::value::Error::custom("No more values"))
            }
        }
    }

    let mut map = TestMap {
        keys: vec![Field::End], // Missing start key
        values: vec![2],
        index: 0,
    };

    let result: Result<(Idx, Idx), serde::de::value::Error> = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_missing_end() {
    struct TestMap {
        keys: Vec<Field>,
        values: Vec<Idx>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = serde::de::value::Error;

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
            T: serde::Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Ok(value as T)
            } else {
                Err(serde::de::value::Error::custom("No more values"))
            }
        }
    }

    let mut map = TestMap {
        keys: vec![Field::Start], // Missing end key
        values: vec![1],
        index: 0,
    };

    let result: Result<(Idx, Idx), serde::de::value::Error> = visit_map(map);
    assert!(result.is_err());
}

