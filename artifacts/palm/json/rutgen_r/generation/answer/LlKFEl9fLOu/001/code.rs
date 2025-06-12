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
    my_map.map.insert("key1".to_string(), serde_json::json!(42));

    let value = my_map.index_mut(&"key1".to_string());
    *value = serde_json::json!(100);

    assert_eq!(my_map.map.get("key1").unwrap(), &serde_json::json!(100));
}

#[should_panic(expected = "no entry found for key")]
#[test]
fn test_index_mut_non_existing_key() {
    let mut my_map = MyMap::default();

    let _value = my_map.index_mut(&"non_existing_key".to_string());
}

