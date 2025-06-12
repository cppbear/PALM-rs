// Answer 0

#[test]
fn test_size_hint_none() {
    struct TestMap;

    impl TestMap {
        fn size_hint(&self) -> Option<usize> {
            None
        }
    }

    let test_map = TestMap;
    assert_eq!(test_map.size_hint(), None);
}

#[test]
fn test_size_hint_with_some() {
    struct TestMap {
        size: usize,
    }

    impl TestMap {
        fn size_hint(&self) -> Option<usize> {
            Some(self.size)
        }
    }

    let test_map = TestMap { size: 5 };
    assert_eq!(test_map.size_hint(), Some(5));
}

