// Answer 0

#[test]
fn test_visit_map_duplicate_start_field() {
    struct TestMap {
        fields: Vec<Field>,
        values: Vec<Idx>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = TestError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.fields.len() {
                let key = self.fields[self.current].clone();
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.current <= self.values.len() {
                let value = self.values[self.current - 1];
                self.current += 1;
                Ok(value as T)
            } else {
                Err(TestError::new("Value error"))
            }
        }
    }

    #[derive(Clone)]
    enum Field {
        Start,
        End,
    }

    struct Idx(i32);

    struct TestError {
        message: String,
    }

    impl TestError {
        fn new(msg: &str) -> Self {
            TestError {
                message: msg.to_string(),
            }
        }
    }

    let mut map = TestMap {
        fields: vec![Field::Start, Field::Start],
        values: vec![Idx(1), Idx(2)],
        current: 0,
    };

    let result = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_missing_start_field() {
    struct TestMap {
        fields: Vec<Field>,
        values: Vec<Idx>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = TestError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.fields.len() {
                let key = self.fields[self.current].clone();
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.current < self.values.len() {
                let value = self.values[self.current];
                self.current += 1;
                Ok(value as T)
            } else {
                Err(TestError::new("Value error"))
            }
        }
    }

    #[derive(Clone)]
    enum Field {
        End,
    }

    struct Idx(i32);

    struct TestError {
        message: String,
    }

    impl TestError {
        fn new(msg: &str) -> Self {
            TestError {
                message: msg.to_string(),
            }
        }
    }

    let mut map = TestMap {
        fields: vec![Field::End],
        values: vec![Idx(2)],
        current: 0,
    };

    let result = visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_missing_end_field() {
    struct TestMap {
        fields: Vec<Field>,
        values: Vec<Idx>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = TestError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.fields.len() {
                let key = self.fields[self.current].clone();
                self.current += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.current < self.values.len() {
                let value = self.values[self.current];
                self.current += 1;
                Ok(value as T)
            } else {
                Err(TestError::new("Value error"))
            }
        }
    }

    #[derive(Clone)]
    enum Field {
        Start,
    }

    struct Idx(i32);

    struct TestError {
        message: String,
    }

    impl TestError {
        fn new(msg: &str) -> Self {
            TestError {
                message: msg.to_string(),
            }
        }
    }

    let mut map = TestMap {
        fields: vec![Field::Start],
        values: vec![Idx(1)],
        current: 0,
    };

    let result = visit_map(map);
    assert!(result.is_err());
}

