// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

struct IndexMap<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K, V> IndexMap<K, V> {
    fn new() -> Self {
        IndexMap { entries: Vec::new() }
    }

    fn insert(&mut self, key: K, value: V) {
        self.entries.push(Entry { key, value });
    }

    fn swap_remove(&mut self, index: usize) -> V {
        let len = self.entries.len();
        if len == 0 {
            panic!("Attempt to swap_remove from an empty map");
        }
        if index >= len {
            panic!("Index out of bounds for swap_remove");
        }
        let last_index = len - 1;
        self.entries.swap(index, last_index);
        self.entries.pop().unwrap().value
    }
}

#[test]
fn test_swap_remove() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    let value = map.swap_remove(1);
    assert_eq!(value, "two");
    assert_eq!(map.entries.len(), 2);
    assert_eq!(map.entries[0].value, "one");
    assert_eq!(map.entries[1].value, "three");
}

#[test]
#[should_panic(expected = "Attempt to swap_remove from an empty map")]
fn test_swap_remove_empty() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.swap_remove(0);
}

#[test]
#[should_panic(expected = "Index out of bounds for swap_remove")]
fn test_swap_remove_out_of_bounds() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.swap_remove(1);
}

