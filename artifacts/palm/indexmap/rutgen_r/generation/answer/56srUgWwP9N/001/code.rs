// Answer 0

#[test]
fn test_from_hash_full_index_not_found() {
    struct TestMap<'a> {
        map: std::collections::HashMap<u64, (&'a str, &'a str)>,
    }

    impl<'a> TestMap<'a> {
        fn new() -> Self {
            Self {
                map: std::collections::HashMap::new(),
            }
        }

        fn index_from_hash<F>(&self, _hash: u64, _is_match: F) -> Option<usize>
        where
            F: FnMut(&'a str) -> bool,
        {
            None
        }

        pub fn from_hash_full<F>(&self, hash: u64, is_match: F) -> Option<(usize, &'a str, &'a str)>
        where
            F: FnMut(&'a str) -> bool,
        {
            let i = self.index_from_hash(hash, is_match)?;
            let (key, value) = self.map.get_index(i)?;
            Some((i, key, value))
        }
    }

    let test_map = TestMap::new();
    let is_match = |key: &str| key == "test_key";

    let result = test_map.from_hash_full(42, is_match);
    assert!(result.is_none());
}

#[test]
fn test_from_hash_full_entry_not_found() {
    struct TestMap<'a> {
        map: std::collections::HashMap<u64, (&'a str, &'a str)>,
    }

    impl<'a> TestMap<'a> {
        fn new() -> Self {
            let mut map = std::collections::HashMap::new();
            map.insert(0, ("test_key1", "value1"));
            Self { map }
        }

        fn index_from_hash<F>(&self, hash: u64, _is_match: F) -> Option<usize>
        where
            F: FnMut(&'a str) -> bool,
        {
            // Return an invalid index
            Some(1)
        }

        pub fn from_hash_full<F>(&self, hash: u64, is_match: F) -> Option<(usize, &'a str, &'a str)>
        where
            F: FnMut(&'a str) -> bool,
        {
            let i = self.index_from_hash(hash, is_match)?;
            let (key, value) = self.map.get_index(i)?;
            Some((i, key, value))
        }
    }

    let test_map = TestMap::new();
    let is_match = |key: &str| key == "test_key";

    let result = test_map.from_hash_full(42, is_match);
    assert!(result.is_none());
}

