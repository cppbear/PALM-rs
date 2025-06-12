// Answer 0

#[derive(Debug)]
struct HashValue {
    value: u64,
}

impl HashValue {
    fn get(&self) -> u64 {
        self.value
    }
}

struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}

struct Indices {
    entries: Vec<u64>,
}

impl Indices {
    fn insert_unique(&mut self, hash: u64, index: usize, max_entries: usize) -> usize {
        self.entries.push(hash);
        index  // Simulating the entry index for simplicity
    }

    fn capacity(&self) -> usize {
        self.entries.capacity()
    }
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

struct Map<K, V> {
    entries: Vec<Bucket<K, V>>,
    indices: Indices,
}

impl<K, V> Map<K, V> {
    fn insert_unique(&mut self, hash: HashValue, key: K, value: V) -> OccupiedEntry<K, V> {
        let i = self.indices.entries.len();
        assert_eq!(i, self.entries.len());
        let entry = self.indices.insert_unique(hash.get(), i, self.entries.len());
        if self.entries.len() == self.entries.capacity() {
            self.entries.reserve(1);
        }
        self.entries.push(Bucket { hash, key, value });
        OccupiedEntry::new(&mut self.entries, entry)
    }
}

#[test]
fn test_insert_unique_when_full_capacity() {
    let mut entries = Vec::with_capacity(2);
    let indices = Indices { entries: vec![] };
    let mut map: Map<String, String> = Map { entries, indices };
    
    map.entries.push(Bucket {
        hash: HashValue { value: 1 },
        key: "key1".to_string(),
        value: "value1".to_string(),
    });
    map.entries.push(Bucket {
        hash: HashValue { value: 2 },
        key: "key2".to_string(),
        value: "value2".to_string(),
    });

    let hash = HashValue { value: 3 };
    let entry = map.insert_unique(hash, "key3".to_string(), "value3".to_string());

    assert_eq!(map.entries.len(), 3);
    assert_eq!(entry.entries.len(), 3);
}

#[test]
#[should_panic]
fn test_insert_unique_panic_when_not_full_capacity() {
    let mut entries = Vec::with_capacity(2);
    let indices = Indices { entries: vec![] };
    let mut map: Map<String, String> = Map { entries, indices };

    map.entries.push(Bucket {
        hash: HashValue { value: 1 },
        key: "key1".to_string(),
        value: "value1".to_string(),
    });

    let hash = HashValue { value: 2 };
    map.insert_unique(hash, "key2".to_string(), "value2".to_string());
}

