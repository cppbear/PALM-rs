// Answer 0

#[derive(Debug)]
struct MockEntry {
    key: u64,
    value: &'static str,
}

struct MockMap {
    entries: Vec<MockEntry>,
}

impl MockMap {
    fn new() -> Self {
        MockMap { entries: Vec::new() }
    }

    fn get_index(&self, index: usize) -> Option<(&u64, &&'static str)> {
        self.entries.get(index).map(|entry| (&entry.key, &entry.value))
    }

    fn index_from_hash<F>(&self, hash: u64, mut is_match: F) -> Option<usize>
    where
        F: FnMut(&u64) -> bool,
    {
        self.entries.iter().position(|entry| is_match(&entry.key))
    }
}

struct MapWrapper<'a> {
    map: &'a MockMap,
}

impl<'a> MapWrapper<'a> {
    pub fn from_hash<F>(self, hash: u64, is_match: F) -> Option<(&'a u64, &'a &'static str)>
    where
        F: FnMut(&u64) -> bool,
    {
        let map = self.map;
        let i = map.index_from_hash(hash, is_match)?;
        map.get_index(i)
    }
}

#[test]
fn test_from_hash_no_match() {
    let mock_map = MockMap {
        entries: vec![
            MockEntry { key: 1, value: "one" },
            MockEntry { key: 2, value: "two" },
        ],
    };
    
    let map_wrapper = MapWrapper { map: &mock_map };

    let result = map_wrapper.from_hash(999, |key| *key == 3);
    
    assert!(result.is_none());
}

#[test]
fn test_from_hash_empty_map() {
    let mock_map = MockMap::new();
    
    let map_wrapper = MapWrapper { map: &mock_map };

    let result = map_wrapper.from_hash(0, |key| *key == 0);
    
    assert!(result.is_none());
}

#[test]
fn test_from_hash_partial_match() {
    let mock_map = MockMap {
        entries: vec![
            MockEntry { key: 1, value: "one" },
            MockEntry { key: 2, value: "two" },
        ],
    };
    
    let map_wrapper = MapWrapper { map: &mock_map };

    // This hash is irrelevant as it won't match
    let result = map_wrapper.from_hash(1, |key| *key == 3);
    
    assert!(result.is_none());
}

