// Answer 0

#[derive(Default)]
struct MyMap {
    map: std::collections::HashMap<String, i32>,
}

impl MyMap {
    pub fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<String, i32> {
        self.map.values_mut()
    }
}

#[test]
fn test_values_mut_empty() {
    let mut my_map = MyMap::default();
    let mut values = my_map.values_mut();
    assert!(values.next().is_none());
}

#[test]
fn test_values_mut_single_entry() {
    let mut my_map = MyMap::default();
    my_map.map.insert("key".to_string(), 42);
    
    {
        let mut values = my_map.values_mut();
        assert_eq!(values.next(), Some(&mut 42));
        assert!(values.next().is_none());
    }
    
    // Ensure the value can be modified
    {
        let mut values = my_map.values_mut();
        if let Some(value) = values.next() {
            *value += 1;
        }
    }
    assert_eq!(my_map.map["key"], 43);
}

#[test]
fn test_values_mut_multiple_entries() {
    let mut my_map = MyMap::default();
    my_map.map.insert("key1".to_string(), 10);
    my_map.map.insert("key2".to_string(), 20);
    
    {
        let mut values = my_map.values_mut();
        let first = values.next().unwrap();
        let second = values.next().unwrap();
        assert!(*first == 10 || *second == 10);
        assert!(*first == 20 || *second == 20);
        
        // Modifying values
        *first += 5;
        *second += 5;
    }
    
    assert_eq!(my_map.map["key1"], 15);
    assert_eq!(my_map.map["key2"], 25);
}

