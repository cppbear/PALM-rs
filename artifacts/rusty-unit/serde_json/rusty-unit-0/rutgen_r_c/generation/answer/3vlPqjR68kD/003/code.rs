// Answer 0

fn test_size_hint_lower_not_equal_upper() {
    struct TestMap {
        iter: std::iter::Once<(usize, Option<usize>)>,
    }

    impl<'de> IntoIterator for TestMap {
        type Item = (usize, Option<usize>);
        type IntoIter = std::iter::Once<(usize, Option<usize>)>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter
        }
    }

    let test_map = TestMap {
        iter: std::iter::once((3, Some(5))),
    };

    let deserializer = MapRefDeserializer {
        iter: test_map.into_iter(),
        value: None,
    };

    assert_eq!(deserializer.size_hint(), None);
}

fn test_size_hint_lower_equal_upper() {
    struct TestMap {
        iter: std::iter::Once<(usize, Option<usize>)>,
    }

    impl<'de> IntoIterator for TestMap {
        type Item = (usize, Option<usize>);
        type IntoIter = std::iter::Once<(usize, Option<usize>)>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter
        }
    }

    let test_map = TestMap {
        iter: std::iter::once((5, Some(5))),
    };

    let deserializer = MapRefDeserializer {
        iter: test_map.into_iter(),
        value: None,
    };

    assert_eq!(deserializer.size_hint(), Some(5));
}

#[test]
fn test_size_hint() {
    test_size_hint_lower_not_equal_upper();
    test_size_hint_lower_equal_upper();
}

