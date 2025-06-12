// Answer 0

#[derive(Debug)]
struct TestEntry {
    key: String,
    value: i32,
}

struct TestMap {
    entries: Vec<TestEntry>,
}

impl TestMap {
    fn get_index(&self, idx: usize) -> Option<(&String, &i32)> {
        self.entries.get(idx).map(|entry| (&entry.key, &entry.value))
    }
    
    fn index_from_hash<F>(&self, hash: u64, is_match: F) -> Option<usize>
    where
        F: FnMut(&String) -> bool,
    {
        let mut is_match = is_match;
        self.entries.iter().position(|entry| is_match(&entry.key))
    }
}

struct TestStruct<'a> {
    map: TestMap,
}

impl<'a> TestStruct<'a> {
    pub fn from_hash_full<F>(&self, hash: u64, is_match: F) -> Option<(usize, &'a String, &'a i32)>
    where
        F: FnMut(&String) -> bool,
    {
        let map = &self.map;
        let i = map.index_from_hash(hash, is_match)?;
        let (key, value) = map.get_index(i)?;
        Some((i, key, value))
    }
}

#[test]
fn test_from_hash_full_success() {
    let entries = vec![
        TestEntry { key: "key1".to_string(), value: 10 },
        TestEntry { key: "key2".to_string(), value: 20 },
        TestEntry { key: "key3".to_string(), value: 30 },
    ];
    let test_map = TestMap { entries };
    
    let test_struct = TestStruct { map: test_map };

    // Assume the hash corresponds to the first key's match condition
    let hash = 0; // Mock hash for "key1"
    let result = test_struct.from_hash_full(hash, |k| k == "key1");
    
    assert_eq!(result, Some((0, &"key1".to_string(), &10)));
}

#[test]
fn test_from_hash_full_no_match() {
    let entries = vec![
        TestEntry { key: "key1".to_string(), value: 10 },
        TestEntry { key: "key2".to_string(), value: 20 },
    ];
    let test_map = TestMap { entries };

    let test_struct = TestStruct { map: test_map };
    
    let hash = 1; // Mock hash for no matching key
    let result = test_struct.from_hash_full(hash, |k| k == "nonexistent");

    assert_eq!(result, None);
}

#[test]
fn test_from_hash_full_index_out_of_bounds() {
    let entries = vec![
        TestEntry { key: "key1".to_string(), value: 10 },
    ];
    let test_map = TestMap { entries };

    let test_struct = TestStruct { map: test_map };
    
    let hash = 2; // Mock hash that leads to out of bounds
    let result = test_struct.from_hash_full(hash, |k| k == "key1");

    assert_eq!(result, None);
}

