// Answer 0

#[test]
fn test_as_mut_slice() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn insert(&mut self, key: i32, value: i32) {
            self.entries.push((key, value));
        }

        fn as_entries_mut(&mut self) -> &mut Vec<(i32, i32)> {
            &mut self.entries
        }

        fn as_mut_slice(&mut self) -> &mut [(i32, i32)] {
            // For the purpose of this test, we'll return the raw slice.
            self.as_entries_mut()
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 10);
    map.insert(2, 20);

    let slice = map.as_mut_slice();
    assert_eq!(slice.len(), 2);
    assert_eq!(slice[0], (1, 10));
    assert_eq!(slice[1], (2, 20));

    // Verify that we can modify the slice
    slice[0].1 = 15;
    slice[1].1 = 25;

    assert_eq!(map.as_mut_slice()[0], (1, 15));
    assert_eq!(map.as_mut_slice()[1], (2, 25));
}

