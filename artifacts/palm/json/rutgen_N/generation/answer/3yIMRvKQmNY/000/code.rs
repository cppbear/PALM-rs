// Answer 0

#[derive(Clone)]
struct MyMap {
    map: std::collections::HashMap<String, i32>,
}

impl MyMap {
    fn new() -> Self {
        MyMap {
            map: std::collections::HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, key: String, value: i32) {
        self.map.insert(key, value);
    }

    pub fn values(&self) -> std::collections::hash_map::Values<String, i32> {
        self.map.values()
    }
}

#[test]
fn test_values_empty() {
    let my_map = MyMap::new();
    let values: Vec<_> = my_map.values().cloned().collect();
    assert_eq!(values, Vec::<i32>::new());
}

#[test]
fn test_values_single() {
    let mut my_map = MyMap::new();
    my_map.insert("key1".into(), 10);
    let values: Vec<_> = my_map.values().cloned().collect();
    assert_eq!(values, vec![10]);
}

#[test]
fn test_values_multiple() {
    let mut my_map = MyMap::new();
    my_map.insert("key1".into(), 10);
    my_map.insert("key2".into(), 20);
    my_map.insert("key3".into(), 30);
    let values: Vec<_> = my_map.values().cloned().collect();
    assert_eq!(values, vec![10, 20, 30]);
}

#[test]
fn test_values_after_remove() {
    let mut my_map = MyMap::new();
    my_map.insert("key1".into(), 10);
    my_map.insert("key2".into(), 20);
    my_map.insert("key3".into(), 30);
    my_map.map.remove("key2");
    let values: Vec<_> = my_map.values().cloned().collect();
    assert_eq!(values, vec![10, 30]);
}

