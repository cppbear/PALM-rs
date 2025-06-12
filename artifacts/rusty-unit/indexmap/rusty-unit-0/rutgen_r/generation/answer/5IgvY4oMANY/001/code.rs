// Answer 0

#[derive(Debug, Clone)]
struct Entry<K, V> {
    key: K,
    value: V,
}

struct Map<K, V> {
    entries: Vec<Entry<K, V>>,
    indices: Vec<usize>,
}

impl<K, V> Map<K, V> {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn swap_remove_finish(&mut self, index: usize) -> (K, V) {
        let entry = self.entries.swap_remove(index);

        if let Some(entry) = self.entries.get(index) {
            let last = self.entries.len();
            // Assume update_index is defined appropriately
            update_index(&mut self.indices, entry.key, last, index);
        }

        (entry.key.clone(), entry.value.clone())
    }
}

fn update_index<K>(indices: &mut Vec<usize>, hash: K, last: usize, index: usize) {
    // Mock implementation that does nothing for testing purposes
}

#[test]
fn test_swap_remove_finish_valid_index() {
    let mut map: Map<i32, String> = Map::new();
    map.entries.push(Entry { key: 1, value: "one".to_string() });
    map.entries.push(Entry { key: 2, value: "two".to_string() });
    map.entries.push(Entry { key: 3, value: "three".to_string() });
    
    let result = map.swap_remove_finish(1);
    assert_eq!(result, (2, "two".to_string()));
    assert_eq!(map.entries.len(), 2);
}

#[test]
fn test_swap_remove_finish_edge_case_last_element() {
    let mut map: Map<i32, String> = Map::new();
    map.entries.push(Entry { key: 1, value: "one".to_string() });
    
    let result = map.swap_remove_finish(0);
    assert_eq!(result, (1, "one".to_string()));
    assert_eq!(map.entries.len(), 0);
}

#[should_panic]
fn test_swap_remove_finish_invalid_index() {
    let mut map: Map<i32, String> = Map::new();
    map.entries.push(Entry { key: 1, value: "one".to_string() });

    let _ = map.swap_remove_finish(1); // This should panic as the index is out of bounds
}

