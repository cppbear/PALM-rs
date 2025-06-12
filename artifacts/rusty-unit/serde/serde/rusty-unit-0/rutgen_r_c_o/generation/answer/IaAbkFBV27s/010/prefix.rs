// Answer 0

#[test]
fn test_visit_map_valid_inputs() {
    struct TestMap {
        current: usize,
        data: Vec<(Field, i32)>,
    }

    impl TestMap {
        fn new(data: Vec<(Field, i32)>) -> Self {
            Self { current: 0, data }
        }
    }

    impl MapAccess<'_> for TestMap {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.data.len() {
                let key = self.data[self.current].0;
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
            if self.current > 0 {
                let value = self.data[self.current - 1].1;
                Ok(value as T)
            } else {
                Err(serde::de::Error::custom("no current value"))
            }
        }
    }

    let data = vec![
        (Field::Start, 1),
        (Field::End, 2),
    ];
    let map = TestMap::new(data);
    let visitor = RangeVisitor { expecting: "a range", phantom: PhantomData::<i32> };
    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_duplicate_start() {
    struct TestMap {
        current: usize,
        data: Vec<(Field, i32)>,
    }

    impl TestMap {
        fn new(data: Vec<(Field, i32)>) -> Self {
            Self { current: 0, data }
        }
    }

    impl MapAccess<'_> for TestMap {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.data.len() {
                let key = self.data[self.current].0;
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
            if self.current > 0 {
                let value = self.data[self.current - 1].1;
                Ok(value as T)
            } else {
                Err(serde::de::Error::custom("no current value"))
            }
        }
    }

    let data = vec![
        (Field::Start, 1),
        (Field::Start, 2),
    ];
    let map = TestMap::new(data);
    let visitor = RangeVisitor { expecting: "a range", phantom: PhantomData::<i32> };
    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_missing_end() {
    struct TestMap {
        current: usize,
        data: Vec<(Field, i32)>,
    }

    impl TestMap {
        fn new(data: Vec<(Field, i32)>) -> Self {
            Self { current: 0, data }
        }
    }

    impl MapAccess<'_> for TestMap {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.data.len() {
                let key = self.data[self.current].0;
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
            if self.current > 0 {
                let value = self.data[self.current - 1].1;
                Ok(value as T)
            } else {
                Err(serde::de::Error::custom("no current value"))
            }
        }
    }

    let data = vec![
        (Field::Start, 1),
    ];
    let map = TestMap::new(data);
    let visitor = RangeVisitor { expecting: "a range", phantom: PhantomData::<i32> };
    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_duplicate_end() {
    struct TestMap {
        current: usize,
        data: Vec<(Field, i32)>,
    }

    impl TestMap {
        fn new(data: Vec<(Field, i32)>) -> Self {
            Self { current: 0, data }
        }
    }

    impl MapAccess<'_> for TestMap {
        type Error = serde::de::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.data.len() {
                let key = self.data[self.current].0;
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
            if self.current > 0 {
                let value = self.data[self.current - 1].1;
                Ok(value as T)
            } else {
                Err(serde::de::Error::custom("no current value"))
            }
        }
    }

    let data = vec![
        (Field::End, 2),
        (Field::End, 3),
    ];
    let map = TestMap::new(data);
    let visitor = RangeVisitor { expecting: "a range", phantom: PhantomData::<i32> };
    let _ = visitor.visit_map(map);
}

