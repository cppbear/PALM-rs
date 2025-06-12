// Answer 0

#[derive(Debug)]
struct Core<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> Core<K, V> {
    fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        if index < self.keys.len() {
            let key = self.keys.swap_remove(index);
            let value = self.values.swap_remove(index);
            Some((key, value))
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.keys.len()
    }

    fn new() -> Self {
        Core {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.keys.push(key);
        self.values.push(value);
    }
}

struct MyMap<K, V> {
    core: Core<K, V>,
}

impl<K, V> MyMap<K, V> {
    fn new() -> Self {
        MyMap {
            core: Core::new(),
        }
    }

    pub fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.core.swap_remove_index(index)
    }
}

#[test]
fn test_swap_remove_index_valid() {
    let mut map = MyMap::new();
    map.core.insert(1, "a");
    map.core.insert(2, "b");
    map.core.insert(3, "c");
    
    let result = map.swap_remove_index(1);
    assert_eq!(result, Some((2, "b")));
    assert_eq!(map.core.len(), 2);
}

#[test]
fn test_swap_remove_index_last_element() {
    let mut map = MyMap::new();
    map.core.insert(1, "a");
    map.core.insert(2, "b");
    
    let result = map.swap_remove_index(1);
    assert_eq!(result, Some((2, "b")));
    assert_eq!(map.core.len(), 1);
}

#[test]
fn test_swap_remove_index_empty() {
    let mut map = MyMap::new();
    
    let result = map.swap_remove_index(0);
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_index_out_of_bounds() {
    let mut map = MyMap::new();
    map.core.insert(1, "a");
    
    let result = map.swap_remove_index(1);
    assert_eq!(result, None);
}

