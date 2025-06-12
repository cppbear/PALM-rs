// Answer 0

#[derive(Debug)]
struct HashValue(usize);

struct Map<K, V> {
    entries: Vec<Entry<K, V>>,
}

struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> Map<K, V> {
    fn shift_insert_unique(&mut self, index: usize, hash: HashValue, key: K, value: V) {
        // Assume this method correctly inserts the entry at the specified index
        self.entries.insert(index, Entry { key, value });
    }
}

struct IndexMap<K, V> {
    map: Map<K, V>,
}

impl<K, V> IndexMap<K, V> {
    pub fn shift_insert_hashed_nocheck(
        mut self,
        index: usize,
        hash: u64,
        key: K,
        value: V,
    ) -> (&'static mut K, &'static mut V) {
        let hash = HashValue(hash as usize);
        self.map.shift_insert_unique(index, hash, key, value);
        // Return mutable references to the inserted key and value
        let entry = &mut self.map.entries[index];
        (&mut entry.key, &mut entry.value)
    }
}

#[test]
fn test_shift_insert_hashed_nocheck() {
    // Prepare the IndexMap with enough capacity to prevent out-of-bounds panic
    let mut map = IndexMap {
        map: Map {
            entries: Vec::new(),
        },
    };

    // Initial values
    let index = 0;
    let hash = 12345;
    let key = String::from("key1");
    let value = String::from("value1");

    // Perform the insertion
    let (inserted_key, inserted_value) = map.shift_insert_hashed_nocheck(index, hash, key.clone(), value.clone());

    // Check that the key and value are as expected
    assert_eq!(*inserted_key, key);
    assert_eq!(*inserted_value, value);
}

#[test]
#[should_panic]
fn test_shift_insert_hashed_nocheck_out_of_bounds() {
    // Prepare the IndexMap with an empty state which will trigger the panic
    let mut map = IndexMap {
        map: Map {
            entries: Vec::new(),
        },
    };

    // Out-of-bounds index
    let index = 1; // This should cause a panic
    let hash = 12345;
    let key = String::from("key2");
    let value = String::from("value2");

    // Attempt the insertion which should panic
    let _ = map.shift_insert_hashed_nocheck(index, hash, key, value);
}

