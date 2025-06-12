// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    struct MapEntry {
        key: i32,
        value: String,
    }

    struct SortedMap {
        entries: Vec<MapEntry>,
    }

    impl SortedMap {
        fn new(entries: Vec<MapEntry>) -> Self {
            SortedMap { entries }
        }

        fn binary_search_by<F, B>(&self, f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32, &String) -> B,
            B: Ord,
        {
            let mut low = 0;
            let mut high = self.entries.len();
            while low < high {
                let mid = low + (high - low) / 2;
                let cmp = f(&self.entries[mid].key, &self.entries[mid].value);
                if cmp == 0 {
                    return Ok(mid);
                } else if cmp < 0 {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }
            Err(low)
        }

        fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32, &String) -> B,
            B: Ord,
        {
            self.binary_search_by(|k, v| f(k, v).cmp(b))
        }
    }

    let map = SortedMap::new(vec![
        MapEntry { key: 1, value: "a".to_string() },
        MapEntry { key: 2, value: "b".to_string() },
        MapEntry { key: 3, value: "c".to_string() },
    ]);

    // Searching for an existing key
    let result = map.binary_search_by_key(&2, |k, _| *k);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_key_not_found() {
    struct MapEntry {
        key: i32,
        value: String,
    }

    struct SortedMap {
        entries: Vec<MapEntry>,
    }

    impl SortedMap {
        fn new(entries: Vec<MapEntry>) -> Self {
            SortedMap { entries }
        }

        fn binary_search_by<F, B>(&self, f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32, &String) -> B,
            B: Ord,
        {
            let mut low = 0;
            let mut high = self.entries.len();
            while low < high {
                let mid = low + (high - low) / 2;
                let cmp = f(&self.entries[mid].key, &self.entries[mid].value);
                if cmp == 0 {
                    return Ok(mid);
                } else if cmp < 0 {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }
            Err(low)
        }

        fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
        where
            F: FnMut(&i32, &String) -> B,
            B: Ord,
        {
            self.binary_search_by(|k, v| f(k, v).cmp(b))
        }
    }

    let map = SortedMap::new(vec![
        MapEntry { key: 1, value: "a".to_string() },
        MapEntry { key: 2, value: "b".to_string() },
        MapEntry { key: 3, value: "c".to_string() },
    ]);

    // Searching for a non-existing key
    let result = map.binary_search_by_key(&4, |k, _| *k);
    assert_eq!(result, Err(3));
}

