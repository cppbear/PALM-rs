// Answer 0

fn test_visit_map_missing_field() {
    use std::marker::PhantomData;
    use crate::de::{MapAccess, Deserializer, Error, Visitor};

    struct MockMapAccess {
        key: Option<Field>,
        finished: bool,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = crate::de::Error; // Assuming the Error type from your context

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.finished {
                Ok(None)
            } else {
                self.finished = true; // Simulate moving past the first key
                Ok(self.key)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(Error::duplicate_field("end")) // Just for coverage, it won't be reached
        }
    }

    let mut mock_map = MockMapAccess {
        key: None,
        finished: false,
    };

    let visitor = RangeToVisitor {
        expecting: "a map with an 'end' field",
        phantom: PhantomData,
    };

    let result: Result<Option<i32>, _> = visitor.visit_map(mock_map);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), <MockMapAccess as MapAccess<'_>>::Error::missing_field("end"));
}

fn test_visit_map_duplicate_field() {
    use std::marker::PhantomData;
    use crate::de::{MapAccess, Deserializer, Error, Visitor};

    struct DuplicateFieldMapAccess {
        finished: bool,
        duplicate_key: bool,
    }

    impl<'de> MapAccess<'de> for DuplicateFieldMapAccess {
        type Error = crate::de::Error; // Assuming the Error type from your context

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if !self.finished {
                self.duplicate_key = true; // Simulate a duplicate key for "end"
                Ok(Some(Field::End))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            if self.duplicate_key {
                self.finished = true; // Set to finished after retrieving value once
                return Err(Error::duplicate_field("end"));
            }
            Err(Error::missing_field("end"))
        }
    }

    let mock_map = DuplicateFieldMapAccess {
        finished: false,
        duplicate_key: false,
    };

    let visitor = RangeToVisitor {
        expecting: "a map with an 'end' field",
        phantom: PhantomData,
    };

    let result: Result<Option<i32>, _> = visitor.visit_map(mock_map);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::duplicate_field("end"));
}

