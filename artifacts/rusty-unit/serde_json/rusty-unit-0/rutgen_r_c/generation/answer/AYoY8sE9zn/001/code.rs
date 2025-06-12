// Answer 0

#[test]
fn test_size_hint_with_non_matching_lower_upper() {
    // Arrange: Create a MapDeserializer with an iterator that has non-matching lower and upper bounds
    struct TestMapIter {
        lower: usize,
        upper: usize,
        count: usize,
    }

    impl Iterator for TestMapIter {
        type Item = (String, Value);

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.lower {
                self.count += 1;
                Some((String::from("key"), Value::Null))
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.lower, Some(self.upper))
        }
    }

    let iter = TestMapIter {
        lower: 2,
        upper: 3,
        count: 0,
    };

    let map_deserializer = MapDeserializer {
        iter,
        value: None,
    };

    // Act: Call the size_hint method
    let result = map_deserializer.size_hint();

    // Assert: Expect the result to be None
    assert_eq!(result, None);
}

#[test]
fn test_size_hint_with_same_lower_upper() {
    // Arrange: Create a MapDeserializer with an iterator that has matching lower and upper bounds
    struct SameLowerUpperIter {
        limit: usize,
        current: usize,
    }

    impl Iterator for SameLowerUpperIter {
        type Item = (String, Value);

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.limit {
                self.current += 1;
                Some((String::from("key"), Value::Null))
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.limit, Some(self.limit))
        }
    }

    let iter = SameLowerUpperIter {
        limit: 3,
        current: 0,
    };

    let map_deserializer = MapDeserializer {
        iter,
        value: None,
    };

    // Act: Call the size_hint method
    let result = map_deserializer.size_hint();

    // Assert: Expect the result to be Some(3)
    assert_eq!(result, Some(3));
}

