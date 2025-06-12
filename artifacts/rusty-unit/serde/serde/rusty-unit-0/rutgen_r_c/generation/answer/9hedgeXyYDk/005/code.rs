// Answer 0

#[test]
fn test_visit_map_with_missing_field() {
    struct TestMap {
        keys: Vec<Field>,
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
            T: Deserialize<'de>,
        {
            Err(serde::de::value::Error::custom("forced error"))
        }
    }

    let map = TestMap { 
        keys: vec![Field::Start], 
        index: 0 
    };

    let visitor = RangeFromVisitor {
        expecting: "expecting test",
        phantom: PhantomData::<i32>,
    };

    let result: Result<i32, _> = visitor.visit_map(map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_with_duplicate_field() {
    struct TestMap {
        keys: Vec<Field>,
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
            T: Deserialize<'de>,
        {
            // this time just return a dummy value
            Ok(0 as T)
        }
    }

    let map = TestMap {
        keys: vec![Field::Start, Field::Start],
        index: 0,
    };

    let visitor = RangeFromVisitor {
        expecting: "expecting test",
        phantom: PhantomData::<i32>,
    };

    let result: Result<i32, _> = visitor.visit_map(map);
    assert!(result.is_err());
}

