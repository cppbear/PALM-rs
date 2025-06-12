// Answer 0

#[derive(Debug)]
struct HashValue(usize);

impl HashValue {
    fn get(&self) -> usize {
        self.0
    }
}

struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}

struct Entries<K, V> {
    entries: Vec<Bucket<K, V>>,
    indices: Vec<usize>,
}

impl<K, V> Entries<K, V> {
    fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            indices: Vec::new(),
        }
    }

    fn increment_indices(&mut self, index: usize, end: usize) {
        for i in index..end {
            self.indices[i] += 1;
        }
    }

    fn reserve_entries(&mut self, additional: usize) {
        self.entries.reserve(additional);
    }

    fn shift_insert_unique(&mut self, index: usize, hash: HashValue, key: K, value: V) {
        let end = self.indices.len();
        assert!(index <= end);
        self.increment_indices(index, end);
        self.indices.insert(index, hash.get()); // Simulating insert_unique
        
        if self.entries.len() == self.entries.capacity() {
            self.reserve_entries(1);
        }
        self.entries.insert(index, Bucket { hash, key, value });
    }
}

#[test]
#[should_panic]
fn test_shift_insert_unique_panic_on_insert_full_capacity() {
    let mut entries = Entries::new(2);
    
    entries.shift_insert_unique(0, HashValue(1), "key1", "value1"); // First insert at index 0
    entries.shift_insert_unique(1, HashValue(2), "key2", "value2"); // Second insert
   
    // This will cause an insert panic because we are at full capacity (2)
    entries.shift_insert_unique(2, HashValue(3), "key3", "value3");
}

#[test]
fn test_shift_insert_unique_insert_at_end_with_capacity() {
    let mut entries = Entries::new(2);
    
    entries.shift_insert_unique(0, HashValue(1), "key1", "value1"); // First insert at index 0
    entries.shift_insert_unique(1, HashValue(2), "key2", "value2"); // Second insert at index 1

    // This does not panic, but we are at full capacity
    let expected_entries_len = 2;
    assert_eq!(entries.entries.len(), expected_entries_len);
    
    // Attempt to insert into the "end" index, incrementing capacity first
    entries.shift_insert_unique(2, HashValue(3), "key3", "value3");
    assert_eq!(entries.entries.len(), expected_entries_len + 1);
}

