// Answer 0

#[test]
fn test_visit_map_missing_field_start() {
    struct TestMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = serde::de::StdError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current];
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
            Err(serde::de::Error::custom("Not applicable value"))
        }
    }

    let keys = vec![Field::Start]; // No keys provided
    let values = vec![1, 2, 3]; // Example values

    let map = TestMap {
        keys,
        values,
        current: 0,
    };

    let visitor = RangeFromVisitor {
        expecting: "an integer",
        phantom: std::marker::PhantomData::<i32>,
    };

    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_duplicate_field_start() {
    struct TestMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        current: usize,
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = serde::de::StdError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current < self.keys.len() {
                let key = self.keys[self.current];
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
            Err(serde::de::Error::custom("Not applicable value"))
        }
    }

    let keys = vec![Field::Start, Field::Start]; // Duplicate Field::Start
    let values = vec![1, 2, 3]; // Example values

    let map = TestMap {
        keys,
        values,
        current: 0,
    };

    let visitor = RangeFromVisitor {
        expecting: "an integer",
        phantom: std::marker::PhantomData::<i32>,
    };

    let _ = visitor.visit_map(map);
}

