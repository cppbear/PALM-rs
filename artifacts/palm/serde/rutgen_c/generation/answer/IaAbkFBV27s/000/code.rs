// Answer 0

#[test]
fn test_visit_map_with_valid_data() {
    struct MapAccessMock {
        data: Vec<(Field, i32)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MapAccessMock {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.data.len() {
                self.index += 1;
                Ok(Some(self.data[self.index - 1].0))
            } else {
                Ok(None)
            }
        }

        fn next_value<B>(&mut self) -> Result<B, Self::Error>
        where
            B: Deserialize<'de>,
        {
            let value = self.data[self.index - 1].1;
            Ok(value.into())
        }
    }

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let data = vec![(Field::Start, 1), (Field::End, 2)];
    let mut map_access = MapAccessMock { data, index: 0 };
    
    let result: Result<(i32, i32), _> = visitor.visit_map(&mut map_access);
    assert_eq!(result.unwrap(), (1, 2));
}

#[test]
fn test_visit_map_missing_start() {
    struct MapAccessMock {
        data: Vec<(Field, i32)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MapAccessMock {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.data.len() {
                self.index += 1;
                Ok(Some(self.data[self.index - 1].0))
            } else {
                Ok(None)
            }
        }

        fn next_value<B>(&mut self) -> Result<B, Self::Error>
        where
            B: Deserialize<'de>,
        {
            let value = self.data[self.index - 1].1;
            Ok(value.into())
        }
    }

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let data = vec![(Field::End, 2)];
    let mut map_access = MapAccessMock { data, index: 0 };

    let result: Result<(i32, i32), _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_duplicate_fields() {
    struct MapAccessMock {
        data: Vec<(Field, i32)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MapAccessMock {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.data.len() {
                self.index += 1;
                Ok(Some(self.data[self.index - 1].0))
            } else {
                Ok(None)
            }
        }

        fn next_value<B>(&mut self) -> Result<B, Self::Error>
        where
            B: Deserialize<'de>,
        {
            let value = self.data[self.index - 1].1;
            Ok(value.into())
        }
    }

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let data = vec![(Field::Start, 1), (Field::Start, 2)];
    let mut map_access = MapAccessMock { data, index: 0 };

    let result: Result<(i32, i32), _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_missing_end() {
    struct MapAccessMock {
        data: Vec<(Field, i32)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MapAccessMock {
        type Error = serde::de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.data.len() {
                self.index += 1;
                Ok(Some(self.data[self.index - 1].0))
            } else {
                Ok(None)
            }
        }

        fn next_value<B>(&mut self) -> Result<B, Self::Error>
        where
            B: Deserialize<'de>,
        {
            let value = self.data[self.index - 1].1;
            Ok(value.into())
        }
    }

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let data = vec![(Field::Start, 1)];
    let mut map_access = MapAccessMock { data, index: 0 };

    let result: Result<(i32, i32), _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err());
}

