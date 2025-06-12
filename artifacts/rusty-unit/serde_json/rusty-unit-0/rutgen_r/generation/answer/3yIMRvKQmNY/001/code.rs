// Answer 0

#[derive(Debug)]
struct MyMap {
    map: std::collections::HashMap<String, i32>,
}

impl MyMap {
    pub fn new() -> Self {
        MyMap {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn values(&self) -> std::collections::hash_map::Values<'_, String, i32> {
        self.map.values()
    }
}

#[test]
fn test_values_with_no_elements() {
    let my_map = MyMap::new();
    let values: Vec<i32> = my_map.values().cloned().collect();
    assert!(values.is_empty());
}

#[test]
fn test_values_with_one_element() {
    let mut my_map = MyMap::new();
    my_map.map.insert("key1".to_string(), 10);
    let values: Vec<i32> = my_map.values().cloned().collect();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0], 10);
}

#[test]
fn test_values_with_multiple_elements() {
    let mut my_map = MyMap::new();
    my_map.map.insert("key1".to_string(), 10);
    my_map.map.insert("key2".to_string(), 20);
    let values: Vec<i32> = my_map.values().cloned().collect();
    assert_eq!(values.len(), 2);
    assert!(values.contains(&10));
    assert!(values.contains(&20));
}

#[test]
fn test_values_with_duplicate_values() {
    let mut my_map = MyMap::new();
    my_map.map.insert("key1".to_string(), 10);
    my_map.map.insert("key2".to_string(), 10);
    let values: Vec<i32> = my_map.values().cloned().collect();
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], 10);
    assert_eq!(values[1], 10);
}

#[test]
fn test_values_with_empty_map() {
    let my_map: MyMap = MyMap::new();
    let values: Vec<i32> = my_map.values().cloned().collect();
    assert!(values.is_empty());
}

