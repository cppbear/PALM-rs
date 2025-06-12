// Answer 0

#[test]
fn test_size_hint_equal_lower_upper() {
    struct TestMap {
        size_hint_result: (usize, Option<usize>),
    }

    impl Iterator for TestMap {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            None // No need for implementation details for this test
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.size_hint_result
        }
    }

    let test_map = TestMap { size_hint_result: (3, Some(3)) };
    let mut map_deserializer = MapDeserializer {
        iter: test_map,
        value: None,
    };

    assert_eq!(map_deserializer.size_hint(), Some(3));
}

#[test]
fn test_size_hint_lower_upper_not_equal() {
    struct TestMap {
        size_hint_result: (usize, Option<usize>),
    }

    impl Iterator for TestMap {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            None // No need for implementation details for this test
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.size_hint_result
        }
    }

    let test_map = TestMap { size_hint_result: (2, None) };
    let mut map_deserializer = MapDeserializer {
        iter: test_map,
        value: None,
    };

    assert_eq!(map_deserializer.size_hint(), None);
}

