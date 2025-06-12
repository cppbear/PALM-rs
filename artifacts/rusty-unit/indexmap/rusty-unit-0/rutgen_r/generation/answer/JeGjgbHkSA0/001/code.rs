// Answer 0

#[test]
fn test_into_keys_non_empty_map() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new(entries: Vec<(i32, String)>) -> Self {
            TestMap { entries }
        }

        fn into_entries(self) -> Vec<(i32, String)> {
            self.entries
        }

        fn into_keys(self: Box<Self>) -> Vec<i32> {
            self.into_entries().into_iter().map(|(k, _)| k).collect()
        }
    }

    let map = Box::new(TestMap::new(vec![(1, "one".to_string()), (2, "two".to_string())]));
    let keys: Vec<i32> = map.into_keys();

    assert_eq!(keys, vec![1, 2]);
}

#[test]
fn test_into_keys_empty_map() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new(entries: Vec<(i32, String)>) -> Self {
            TestMap { entries }
        }

        fn into_entries(self) -> Vec<(i32, String)> {
            self.entries
        }

        fn into_keys(self: Box<Self>) -> Vec<i32> {
            self.into_entries().into_iter().map(|(k, _)| k).collect()
        }
    }

    let map = Box::new(TestMap::new(vec![]));
    let keys: Vec<i32> = map.into_keys();

    assert_eq!(keys, vec![]);
}

#[test]
#[should_panic]
fn test_into_keys_panic_condition() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new(entries: Vec<(i32, String)>) -> Self {
            TestMap { entries }
        }

        fn into_entries(self) -> Vec<(i32, String)> {
            self.entries
        }

        fn into_keys(self: Box<Self>) -> Vec<i32> {
            self.entries.push((3, "three".to_string())); // This will cause an invalid state
            self.into_entries().into_iter().map(|(k, _)| k).collect()
        }
    }

    let map = Box::new(TestMap::new(vec![(1, "one".to_string())]));
    let _ = map.into_keys(); // This triggers the panic condition
}

