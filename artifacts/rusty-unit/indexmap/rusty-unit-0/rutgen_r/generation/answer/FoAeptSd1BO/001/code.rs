// Answer 0

#[derive(Debug)]
struct Entry<V> {
    value: V,
}

#[derive(Debug)]
struct Map<V> {
    entries: Vec<Entry<V>>,
}

impl<V> Map<V> {
    fn new() -> Self {
        Map { entries: Vec::new() }
    }
    
    fn push(&mut self, value: V) {
        self.entries.push(Entry { value });
    }
    
    fn index(&self) -> usize {
        self.entries.len() - 1 // returns last index
    }

    pub fn get(&self) -> &V {
        &self.entries[self.index()].value
    }
}

#[test]
fn test_get_valid() {
    let mut map: Map<i32> = Map::new();
    map.push(42);
    assert_eq!(*map.get(), 42);
}

#[test]
#[should_panic]
fn test_get_empty() {
    let map: Map<i32> = Map::new();
    let _ = map.get(); // This should panic as the map is empty.
}

#[test]
fn test_get_multiple_entries() {
    let mut map: Map<&str> = Map::new();
    map.push("first");
    map.push("second");
    assert_eq!(map.get(), &"second");
}

#[test]
#[should_panic]
fn test_get_after_clear() {
    let mut map: Map<i32> = Map::new();
    map.push(1);
    map.push(2);
    map.entries.clear(); // Clear all entries
    let _ = map.get(); // This should panic.
}

