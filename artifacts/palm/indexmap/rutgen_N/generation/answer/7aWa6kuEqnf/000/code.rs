// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

struct MyMap<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K, V> MyMap<K, V> {
    fn new() -> Self {
        MyMap { entries: Vec::new() }
    }

    fn push(&mut self, key: K, value: V) {
        self.entries.push(Entry { key, value });
    }

    fn swap_remove_entry(&mut self, index: usize) -> Option<Entry<K, V>> {
        if index < self.entries.len() {
            let last_index = self.entries.len() - 1;
            self.entries.swap(index, last_index);
            Some(self.entries.pop().unwrap())
        } else {
            None
        }
    }
}

impl<K, V> Entry<K, V> {
    fn swap_remove(self) -> V {
        let value = self.value;
        value
    }
}

#[test]
fn test_swap_remove_single_element() {
    let mut map = MyMap::new();
    map.push(1, "a");
    let entry = map.entries.remove(0);
    assert_eq!(entry.swap_remove(), "a");
}

#[test]
fn test_swap_remove_multiple_elements() {
    let mut map = MyMap::new();
    map.push(1, "a");
    map.push(2, "b");
    let entry = map.entries.remove(1);
    assert_eq!(entry.swap_remove(), "b");
    assert_eq!(map.entries.len(), 1);
}

#[test]
fn test_swap_remove_empty_map() {
    let mut map: MyMap<i32, &str> = MyMap::new();
    assert!(map.swap_remove_entry(0).is_none());
}

#[test]
fn test_swap_remove_out_of_bounds() {
    let mut map = MyMap::new();
    map.push(1, "a");
    assert!(map.swap_remove_entry(1).is_none());
}

