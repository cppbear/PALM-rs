// Answer 0

#[test]
fn test_sort_unstable_by() {
    struct TestEntry {
        key: i32,
        value: String,
    }

    impl TestEntry {
        fn new(key: i32, value: &str) -> Self {
            TestEntry {
                key,
                value: value.to_string(),
            }
        }
    }

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: Vec::new(),
            }
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut Vec<TestEntry>),
        {
            f(&mut self.entries);
        }

        fn sort_unstable_by<F>(&mut self, mut cmp: F)
        where
            F: FnMut(&i32, &String, &i32, &String) -> Ordering,
        {
            self.with_entries(move |entries| {
                entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            });
        }
    }

    // Test case: Sort with multiple distinct keys
    let mut map = TestMap::new();
    map.entries.push(TestEntry::new(3, "three"));
    map.entries.push(TestEntry::new(1, "one"));
    map.entries.push(TestEntry::new(2, "two"));

    map.sort_unstable_by(|k1, v1, k2, v2| k1.cmp(k2));

    assert_eq!(map.entries[0].key, 1);
    assert_eq!(map.entries[1].key, 2);
    assert_eq!(map.entries[2].key, 3);

    // Test case: Sort with duplicate keys
    let mut map_dup = TestMap::new();
    map_dup.entries.push(TestEntry::new(2, "two"));
    map_dup.entries.push(TestEntry::new(1, "one"));
    map_dup.entries.push(TestEntry::new(2, "another two"));

    map_dup.sort_unstable_by(|_, v1, _, v2| v1.cmp(v2));

    assert_eq!(map_dup.entries[0].value, "another two");
    assert_eq!(map_dup.entries[1].value, "one");
    assert_eq!(map_dup.entries[2].value, "two");

    // Test case: Already sorted map
    let mut map_sorted = TestMap::new();
    map_sorted.entries.push(TestEntry::new(1, "one"));
    map_sorted.entries.push(TestEntry::new(2, "two"));
    map_sorted.entries.push(TestEntry::new(3, "three"));

    map_sorted.sort_unstable_by(|k1, _, k2, _| k1.cmp(k2));

    assert_eq!(map_sorted.entries[0].key, 1);
    assert_eq!(map_sorted.entries[1].key, 2);
    assert_eq!(map_sorted.entries[2].key, 3);

    // Test case: Empty map
    let mut map_empty = TestMap::new();
    
    map_empty.sort_unstable_by(|_, _, _, _| Ordering::Equal);
    
    assert!(map_empty.entries.is_empty());
}

