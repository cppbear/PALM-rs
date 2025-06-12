// Answer 0

#[test]
fn test_visit_map_duplicate_start_field() {
    struct MockMap {
        keys: Vec<Field>,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.keys.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.keys.remove(0)))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(MockError)
        }
    }

    struct MockError;

    impl Error for MockError {
        fn invalid_type(_unexpected: Unexpected, _visitor: &dyn Visitor<'_>) -> Self {
            MockError
        }

        fn duplicate_field(_field: &str) -> Self {
            MockError
        }

        fn missing_field(_field: &str) -> Self {
            MockError
        }
    }

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: PhantomData,
    };

    let mut map = MockMap {
        keys: vec![Field::Start, Field::Start],
    };

    let result = visitor.visit_map(&mut map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_missing_start_field() {
    struct MockMap {
        keys: Vec<Field>,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.keys.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.keys.remove(0)))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(MockError)
        }
    }

    struct MockError;

    impl Error for MockError {
        fn invalid_type(_unexpected: Unexpected, _visitor: &dyn Visitor<'_>) -> Self {
            MockError
        }

        fn duplicate_field(_field: &str) -> Self {
            MockError
        }

        fn missing_field(_field: &str) -> Self {
            MockError
        }
    }

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: PhantomData,
    };

    let mut map = MockMap {
        keys: vec![Field::End],
    };

    let result = visitor.visit_map(&mut map);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_missing_end_field() {
    struct MockMap {
        keys: Vec<Field>,
    }

    impl<'de> MapAccess<'de> for MockMap {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.keys.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.keys.remove(0)))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(MockError)
        }
    }

    struct MockError;

    impl Error for MockError {
        fn invalid_type(_unexpected: Unexpected, _visitor: &dyn Visitor<'_>) -> Self {
            MockError
        }

        fn duplicate_field(_field: &str) -> Self {
            MockError
        }

        fn missing_field(_field: &str) -> Self {
            MockError
        }
    }

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: PhantomData,
    };

    let mut map = MockMap {
        keys: vec![Field::Start],
    };

    let result = visitor.visit_map(&mut map);
    assert!(result.is_err());
}

