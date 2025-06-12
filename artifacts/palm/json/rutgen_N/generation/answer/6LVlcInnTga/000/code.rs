// Answer 0

#[test]
fn test_iter_non_empty_map() {
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

        pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, i32> {
            self.map.iter()
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert("a".to_string(), 1);
    my_map.insert("b".to_string(), 2);
    
    let mut iter = my_map.iter();
    assert_eq!(iter.next(), Some((&"a".to_string(), &1)));
    assert_eq!(iter.next(), Some((&"b".to_string(), &2)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_empty_map() {
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

        pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, i32> {
            self.map.iter()
        }
    }

    let my_map = MyMap::new();
    let mut iter = my_map.iter();
    assert_eq!(iter.next(), None);
}

