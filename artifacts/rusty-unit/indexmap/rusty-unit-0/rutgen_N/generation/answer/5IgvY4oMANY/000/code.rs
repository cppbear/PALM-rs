// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
    hash: usize,
}

#[derive(Debug)]
struct IndexMap<K, V> {
    entries: Vec<Entry<K, V>>,
    indices: Vec<usize>,
}

impl<K, V> IndexMap<K, V> {
    fn new() -> Self {
        IndexMap {
            entries: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn swap_remove_finish(&mut self, index: usize) -> (K, V) {
        let entry = self.entries.swap_remove(index);

        if let Some(entry) = self.entries.get(index) {
            let last = self.entries.len();
            update_index(self.indices, entry.hash, last, index);
        }

        (entry.key, entry.value)
    }
}

fn update_index<K>(indices: &mut Vec<usize>, hash: usize, last: usize, index: usize) {
    // Dummy implementation; just to satisfy the function call.
    if last > index {
        indices[index] = hash; // Simulating an index update
    }
}

#[test]
fn test_swap_remove_finish_non_empty() {
    let mut index_map = IndexMap::new();
    index_map.entries.push(Entry { key: "a", value: 1, hash: 123 });
    index_map.entries.push(Entry { key: "b", value: 2, hash: 234 });
    
    let (key, value) = index_map.swap_remove_finish(0);
    
    assert_eq!(key, "a");
    assert_eq!(value, 1);
    assert_eq!(index_map.entries.len(), 1);
    assert_eq!(index_map.entries[0].key, "b");
}

#[test]
fn test_swap_remove_finish_single_element() {
    let mut index_map = IndexMap::new();
    index_map.entries.push(Entry { key: "a", value: 1, hash: 123 });

    let (key, value) = index_map.swap_remove_finish(0);

    assert_eq!(key, "a");
    assert_eq!(value, 1);
    assert_eq!(index_map.entries.len(), 0);
}

#[should_panic]
fn test_swap_remove_finish_out_of_bounds() {
    let mut index_map = IndexMap::new();
    index_map.entries.push(Entry { key: "a", value: 1, hash: 123 });

    let _ = index_map.swap_remove_finish(1); // Index out of bounds
}

