// Answer 0

#[test]
fn test_shift_remove_full_single_entry() {
    use std::hash::{BuildHasherDefault, Hash};
    use std::collections::hash_map::DefaultHasher;

    struct TestKey {
        id: usize,
    }

    impl Hash for TestKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    impl PartialEq for TestKey {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    struct TestEntry {
        key: TestKey,
        value: String,
    }

    impl TestEntry {
        fn new(id: usize, value: &str) -> Self {
            Self {
                key: TestKey { id },
                value: value.to_string(),
            }
        }
    }

    struct MockIndexMap {
        entries: Vec<TestEntry>,
    }

    impl MockIndexMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn insert(&mut self, entry: TestEntry) {
            self.entries.push(entry);
        }

        fn as_entries(&self) -> &[TestEntry] {
            &self.entries
        }

        fn hash<Q: ?Sized + Hash>(&self, key: &Q) -> usize {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            hasher.finish() as usize
        }

        fn core_pop(&mut self) -> Option<(TestKey, String)> {
            self.entries.pop().map(|entry| (entry.key, entry.value))
        }

        pub fn shift_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, TestKey, String)>
        where
            Q: ?Sized + Hash + PartialEq<TestKey>,
        {
            match self.as_entries() {
                [x] if key == &x.key => {
                    let (k, v) = self.core_pop()?;
                    Some((0, k, v))
                }
                [_] | [] => None,
                _ => None,
            }
        }
    }

    // Setup the test case with a single entry
    let mut map = MockIndexMap::new();
    let key = TestKey { id: 1 };
    let entry = TestEntry::new(1, "value1");
    map.insert(entry);

    // Perform the test
    if let Some((index, k, v)) = map.shift_remove_full(&key) {
        assert_eq!(index, 0);
        assert_eq!(k.id, 1);
        assert_eq!(v, "value1");
    } else {
        panic!("Expected some value to be returned");
    }
}

