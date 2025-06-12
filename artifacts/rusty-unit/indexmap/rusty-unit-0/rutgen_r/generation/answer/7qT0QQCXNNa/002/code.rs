// Answer 0

#[test]
fn test_from_hash_valid_entry() {
    struct TestEntry {
        map: indexmap::IndexMap<u64, String>,
    }

    impl TestEntry {
        fn new() -> Self {
            let mut map = indexmap::IndexMap::new();
            map.insert(1, "one".to_string());
            map.insert(2, "two".to_string());
            map.insert(3, "three".to_string());
            TestEntry { map }
        }

        fn index_from_hash<F>(&self, hash: u64, mut is_match: F) -> Option<usize>
        where
            F: FnMut(&u64) -> bool,
        {
            self.map.keys().position(|key| {
                let key_hash = (*key as u64).wrapping_hash();
                key_hash == hash && is_match(key)
            })
        }

        pub fn from_hash<F>(&self, hash: u64, is_match: F) -> Option<(&u64, &String)>
        where
            F: FnMut(&u64) -> bool,
        {
            let i = self.index_from_hash(hash, is_match)?;
            self.map.get_index(i)
        }
    }

    let entry = TestEntry::new();

    let result = entry.from_hash(1.wrapping_hash(), |&k| k == &1);
    assert!(result.is_some());
    assert_eq!(result, Some((&1, &"one".to_string())));

    let result = entry.from_hash(2.wrapping_hash(), |&k| k == &2);
    assert!(result.is_some());
    assert_eq!(result, Some((&2, &"two".to_string())));
}

#[test]
fn test_from_hash_no_match() {
    struct TestEntry {
        map: indexmap::IndexMap<u64, String>,
    }

    impl TestEntry {
        fn new() -> Self {
            let mut map = indexmap::IndexMap::new();
            map.insert(1, "one".to_string());
            map.insert(2, "two".to_string());
            map.insert(3, "three".to_string());
            TestEntry { map }
        }

        fn index_from_hash<F>(&self, hash: u64, mut is_match: F) -> Option<usize>
        where
            F: FnMut(&u64) -> bool,
        {
            self.map.keys().position(|key| {
                let key_hash = (*key as u64).wrapping_hash();
                key_hash == hash && is_match(key)
            })
        }

        pub fn from_hash<F>(&self, hash: u64, is_match: F) -> Option<(&u64, &String)>
        where
            F: FnMut(&u64) -> bool,
        {
            let i = self.index_from_hash(hash, is_match)?;
            self.map.get_index(i)
        }
    }

    let entry = TestEntry::new();

    let result = entry.from_hash(4.wrapping_hash(), |&k| k == &4);
    assert!(result.is_none());
}

#[test]
#[should_panic]
fn test_from_hash_panics_with_invalid_hash() {
    struct TestEntry {
        map: indexmap::IndexMap<u64, String>,
    }

    impl TestEntry {
        fn new() -> Self {
            let mut map = indexmap::IndexMap::new();
            map.insert(1, "one".to_string());
            map.insert(2, "two".to_string());
            map.insert(3, "three".to_string());
            TestEntry { map }
        }

        fn index_from_hash<F>(&self, _hash: u64, _is_match: F) -> Option<usize>
        where
            F: FnMut(&u64) -> bool,
        {
            None
        }

        pub fn from_hash<F>(&self, hash: u64, is_match: F) -> Option<(&u64, &String)>
        where
            F: FnMut(&u64) -> bool,
        {
            let i = self.index_from_hash(hash, is_match)?;
            self.map.get_index(i)
        }
    }

    let entry = TestEntry::new();
    let _ = entry.from_hash(0, |&k| k == &0); // This should panic
}

