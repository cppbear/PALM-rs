// Answer 0

#[test]
fn test_iter_mut_empty_map() {
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, i32> {
            self.map.iter_mut()
        }
    }

    let mut my_map = MyMap::new();
    let mut iter = my_map.iter_mut();
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_mut_single_entry() {
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: String, value: i32) {
            self.map.insert(key, value);
        }

        pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, i32> {
            self.map.iter_mut()
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), 10);
    
    let mut iter = my_map.iter_mut();
    if let Some((key, value)) = iter.next() {
        assert_eq!(key, "key1");
        *value += 5;  // Modify the value through the iterator
    }
    
    assert_eq!(my_map.map["key1"], 15);
}

#[test]
fn test_iter_mut_multiple_entries() {
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, i32>,
    }

    impl MyMap {
        pub fn new() -> Self {
            MyMap {
                map: HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: String, value: i32) {
            self.map.insert(key, value);
        }

        pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, i32> {
            self.map.iter_mut()
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert("key1".to_string(), 10);
    my_map.insert("key2".to_string(), 20);
    
    let mut iter = my_map.iter_mut();
    if let Some((key, value)) = iter.next() {
        assert_eq!(key, "key1");
        *value += 5;  // Modify the value through the iterator
    }
    
    if let Some((key, value)) = iter.next() {
        assert_eq!(key, "key2");
        *value += 10;  // Modify the value through the iterator
    }
    
    assert_eq!(my_map.map["key1"], 15);
    assert_eq!(my_map.map["key2"], 30);
}

