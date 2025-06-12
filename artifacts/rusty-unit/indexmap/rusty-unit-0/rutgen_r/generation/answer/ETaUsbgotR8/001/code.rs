// Answer 0

#[test]
fn test_get_full_mut2_no_entry() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    struct TestEntry {
        key: String,
        value: i32,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            for (i, entry) in self.entries.iter().enumerate() {
                if entry.key == *key {
                    return Some(i);
                }
            }
            None
        }

        fn as_entries_mut(&mut self) -> &mut Vec<TestEntry> {
            &mut self.entries
        }

        fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut String, &mut i32)>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &mut self.as_entries_mut()[i];
                Some((i, &mut entry.key, &mut entry.value))
            } else {
                None
            }
        }
    }

    impl<Q: ?Sized + Hash> Equivalent<Q> for String {
        fn equivalent(&self, other: &Q) -> bool {
            self == other
        }
    }

    let mut test_map = TestMap::new();
    test_map.entries.push(TestEntry {
        key: "existing_key".to_string(),
        value: 42,
    });

    let result: Option<(usize, &mut String, &mut i32)> = test_map.get_full_mut2(&"non_existing_key");
    assert!(result.is_none());
}

