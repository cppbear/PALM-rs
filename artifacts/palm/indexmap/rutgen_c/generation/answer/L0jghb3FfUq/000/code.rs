// Answer 0

#[test]
fn test_sort_unstable_keys() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new(entries: Vec<(i32, i32)>) -> Self {
            TestMap { entries }
        }

        fn sort_unstable_keys(&mut self) {
            self.entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        }

        fn as_slice(&self) -> &[(i32, i32)] {
            &self.entries
        }
    }

    let mut map = TestMap::new(vec![(3, 100), (1, 200), (2, 300)]);
    map.sort_unstable_keys();
    assert_eq!(map.as_slice(), &[(1, 200), (2, 300), (3, 100)]);

    let mut map_with_duplicates = TestMap::new(vec![(3, 100), (1, 200), (2, 300), (2, 400)]);
    map_with_duplicates.sort_unstable_keys();
    assert_eq!(map_with_duplicates.as_slice(), &[(1, 200), (2, 300), (2, 400), (3, 100)]);
}

#[test]
fn test_sort_unstable_keys_empty() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new(entries: Vec<(i32, i32)>) -> Self {
            TestMap { entries }
        }

        fn sort_unstable_keys(&mut self) {
            self.entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        }

        fn as_slice(&self) -> &[(i32, i32)] {
            &self.entries
        }
    }

    let mut empty_map = TestMap::new(vec![]);
    empty_map.sort_unstable_keys();
    assert_eq!(empty_map.as_slice(), &[]);
}

