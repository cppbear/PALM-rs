// Answer 0

#[test]
fn test_visit_map_duplicate_field_start() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<Result<i32, serde_json::Error>>,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = serde_json::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.keys.is_empty() {
                return Ok(None);
            }
            Ok(Some(self.keys.remove(0)))
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            match self.values.remove(0) {
                Ok(value) => Ok(value as V),
                Err(err) => Err(err),
            }
        }
    }

    let visitor = RangeVisitor::<i32> {
        expecting: "an i32 type",
        phantom: std::marker::PhantomData,
    };
    let map = MockMap {
        keys: vec![Field::Start, Field::Start],
        values: vec![Ok(1)],
    };
    let result = visitor.visit_map(map);
}

#[test]
fn test_visit_map_missing_field_end() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<Result<i32, serde_json::Error>>,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = serde_json::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.keys.is_empty() {
                return Ok(None);
            }
            Ok(Some(self.keys.remove(0)))
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            match self.values.remove(0) {
                Ok(value) => Ok(value as V),
                Err(err) => Err(err),
            }
        }
    }

    let visitor = RangeVisitor::<i32> {
        expecting: "an i32 type",
        phantom: std::marker::PhantomData,
    };
    let map = MockMap {
        keys: vec![Field::Start],
        values: vec![Ok(1)],
    };
    let result = visitor.visit_map(map);
}

#[test]
fn test_visit_map_error_on_value() {
    struct MockMap {
        keys: Vec<Field>,
        values: Vec<Result<i32, serde_json::Error>>,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = serde_json::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.keys.is_empty() {
                return Ok(None);
            }
            Ok(Some(self.keys.remove(0)))
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            match self.values.remove(0) {
                Ok(value) => Ok(value as V),
                Err(err) => Err(err),
            }
        }
    }

    let visitor = RangeVisitor::<i32> {
        expecting: "an i32 type",
        phantom: std::marker::PhantomData,
    };
    let map = MockMap {
        keys: vec![Field::Start, Field::End],
        values: vec![Ok(1), Err(serde_json::Error::custom("error"))],
    };
    let result = visitor.visit_map(map);
}

