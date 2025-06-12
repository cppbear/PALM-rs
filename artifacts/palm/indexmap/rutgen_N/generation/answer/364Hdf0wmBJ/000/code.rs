// Answer 0

#[derive(Debug)]
struct HashValue {
    value: usize,
}

impl HashValue {
    fn get(&self) -> usize {
        self.value
    }
}

#[derive(Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}

struct OccupiedEntry<'a, K, V> {
    entries: &'a mut Vec<Bucket<K, V>>,
    index: usize,
}

impl<'a, K, V> OccupiedEntry<'a, K, V> {
    fn new(entries: &'a mut Vec<Bucket<K, V>>, index: usize) -> Self {
        OccupiedEntry { entries, index }
    }
}

struct IndexMap<K, V> {
    entries: Vec<Bucket<K, V>>,
    indices: Vec<usize>,
}

impl<K, V> IndexMap<K, V> {
    fn new() -> Self {
        IndexMap {
            entries: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn insert_unique(
        &mut self,
        hash: HashValue,
        key: K,
        value: V,
    ) -> OccupiedEntry<K, V> {
        let i = self.indices.len();
        debug_assert_eq!(i, self.entries.len());
        let entry = self.indices.len(); // Simulating unique index insertion
        self.entries.push(Bucket { hash, key, value });
        OccupiedEntry::new(&mut self.entries, entry)
    }

    fn reserve_entries(&mut self, additional: usize) {
        self.entries.reserve(additional);
    }
}

#[test]
fn test_insert_unique() {
    let mut index_map: IndexMap<i32, String> = IndexMap::new();
    let hash_value = HashValue { value: 1 };
    let entry = index_map.insert_unique(hash_value, 42, String::from("value"));
    assert_eq!(index_map.entries.len(), 1);
    assert_eq!(entry.index, 0);
    assert_eq!(index_map.entries[0].key, 42);
    assert_eq!(index_map.entries[0].value, "value");
}

#[test]
fn test_insert_multiple_unique() {
    let mut index_map: IndexMap<i32, String> = IndexMap::new();
    let hash1 = HashValue { value: 1 };
    let hash2 = HashValue { value: 2 };
    
    let entry1 = index_map.insert_unique(hash1, 10, String::from("first"));
    let entry2 = index_map.insert_unique(hash2, 20, String::from("second"));
    
    assert_eq!(index_map.entries.len(), 2);
    assert_eq!(entry1.index, 0);
    assert_eq!(index_map.entries[0].key, 10);
    assert_eq!(index_map.entries[0].value, "first");
    
    assert_eq!(entry2.index, 1);
    assert_eq!(index_map.entries[1].key, 20);
    assert_eq!(index_map.entries[1].value, "second");
}

