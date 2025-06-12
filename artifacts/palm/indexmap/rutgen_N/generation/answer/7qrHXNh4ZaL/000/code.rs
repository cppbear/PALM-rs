// Answer 0

#[test]
fn test_as_entries_mut() {
    struct TestEntry {
        value: i32,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }

        fn as_entries_mut(&mut self) -> &mut [TestEntry] {
            &mut self.entries
        }
    }

    let mut map = TestMap::new();
    map.entries.push(TestEntry { value: 1 });
    map.entries.push(TestEntry { value: 2 });

    let entries_mut = map.as_entries_mut();
    entries_mut[0].value = 10;

    assert_eq!(entries_mut[0].value, 10);
    assert_eq!(map.entries[0].value, 10);
}

#[test]
fn test_as_entries_mut_empty() {
    struct TestEntry {
        value: i32,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }

        fn as_entries_mut(&mut self) -> &mut [TestEntry] {
            &mut self.entries
        }
    }

    let mut map = TestMap::new();
    let entries_mut = map.as_entries_mut();

    assert_eq!(entries_mut.len(), 0);
}

