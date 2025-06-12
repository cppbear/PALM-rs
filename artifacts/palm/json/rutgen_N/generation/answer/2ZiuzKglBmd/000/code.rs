// Answer 0

#[derive(Default)]
struct MyMap {
    map: std::collections::HashMap<String, String>,
}

impl MyMap {
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

#[test]
fn test_clear_removes_all_values() {
    let mut my_map = MyMap::default();
    
    my_map.map.insert("key1".to_string(), "value1".to_string());
    my_map.map.insert("key2".to_string(), "value2".to_string());
    
    assert_eq!(my_map.map.len(), 2);

    my_map.clear();
    
    assert_eq!(my_map.map.len(), 0);
}

#[test]
fn test_clear_on_empty_map() {
    let mut my_map = MyMap::default();
    
    assert_eq!(my_map.map.len(), 0);
    
    my_map.clear();
    
    assert_eq!(my_map.map.len(), 0);
}

