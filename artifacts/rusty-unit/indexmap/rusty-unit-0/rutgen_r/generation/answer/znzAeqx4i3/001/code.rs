// Answer 0

#[test]
fn test_as_entries_non_empty() {
    struct TestMap {
        entries: Vec<i32>,
    }

    impl TestMap {
        fn new(entries: Vec<i32>) -> Self {
            TestMap { entries }
        }

        fn as_entries(&self) -> &[i32] {
            &self.entries
        }
    }

    let map = TestMap::new(vec![1, 2, 3, 4, 5]);
    let entries = map.as_entries();
    assert_eq!(entries, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_as_entries_empty() {
    struct TestMap {
        entries: Vec<i32>,
    }

    impl TestMap {
        fn new(entries: Vec<i32>) -> Self {
            TestMap { entries }
        }

        fn as_entries(&self) -> &[i32] {
            &self.entries
        }
    }

    let map = TestMap::new(vec![]);
    let entries = map.as_entries();
    assert_eq!(entries, &[]);
}

#[test]
fn test_as_entries_single_element() {
    struct TestMap {
        entries: Vec<i32>,
    }

    impl TestMap {
        fn new(entries: Vec<i32>) -> Self {
            TestMap { entries }
        }

        fn as_entries(&self) -> &[i32] {
            &self.entries
        }
    }

    let map = TestMap::new(vec![42]);
    let entries = map.as_entries();
    assert_eq!(entries, &[42]);
}

