// Answer 0

#[derive(Default)]
struct MyMap {
    core: std::collections::HashMap<String, String>,
}

impl MyMap {
    pub fn clear(&mut self) {
        self.core.clear();
    }
}

#[test]
fn test_clear_empty_map() {
    let mut map = MyMap::default();
    map.clear();
    assert!(map.core.is_empty());
}

#[test]
fn test_clear_non_empty_map() {
    let mut map = MyMap::default();
    map.core.insert("key".to_string(), "value".to_string());
    map.clear();
    assert!(map.core.is_empty());
}

#[test]
fn test_clear_preserve_capacity() {
    let mut map = MyMap::default();
    for i in 0..100 {
        map.core.insert(format!("key{}", i), format!("value{}", i));
    }
    let original_capacity = map.core.capacity();
    map.clear();
    assert!(map.core.is_empty());
    assert_eq!(original_capacity, map.core.capacity());
}

