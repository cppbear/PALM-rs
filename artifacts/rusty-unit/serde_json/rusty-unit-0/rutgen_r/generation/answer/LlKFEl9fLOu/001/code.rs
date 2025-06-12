// Answer 0

#[derive(Default)]
struct MyMap {
    map: std::collections::HashMap<String, serde_json::Value>,
}

impl MyMap {
    fn index_mut(&mut self, index: &String) -> &mut serde_json::Value {
        self.map.get_mut(index).expect("no entry found for key")
    }
}

#[test]
fn test_index_mut_existing_key() {
    let mut my_map = MyMap::default();
    my_map.map.insert("key1".to_string(), serde_json::json!(1));
    
    let value = my_map.index_mut(&"key1".to_string());
    *value = serde_json::json!(2);
    
    assert_eq!(my_map.map["key1"], serde_json::json!(2));
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_non_existing_key() {
    let mut my_map = MyMap::default();
    my_map.index_mut(&"non_existing_key".to_string());
}

#[test]
fn test_index_mut_empty_map() {
    let mut my_map = MyMap::default();
    
    let value = my_map.map.entry("new_key".to_string()).or_insert(serde_json::json!(0));
    *value = serde_json::json!(3);
    
    let returned_value = my_map.index_mut(&"new_key".to_string());
    assert_eq!(*returned_value, serde_json::json!(3));
}

