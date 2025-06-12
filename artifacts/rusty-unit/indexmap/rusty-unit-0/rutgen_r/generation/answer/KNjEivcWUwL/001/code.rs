// Answer 0

#[derive(Debug, PartialEq, Eq, Hash)]
struct Key(i32);

#[derive(Debug)]
struct Value(String);

struct MyMap {
    data: indexmap::IndexMap<Key, Value>,
}

impl MyMap {
    fn new() -> Self {
        MyMap {
            data: indexmap::IndexMap::new(),
        }
    }

    fn insert(&mut self, key: Key, value: Value) {
        self.data.insert(key, value);
    }

    fn swap_remove(&mut self, key: &Key) -> Option<Value> {
        self.data.swap_remove(key)
    }
}

#[test]
fn test_swap_remove_existing_key() {
    let mut my_map = MyMap::new();
    my_map.insert(Key(1), Value("One".to_string()));
    my_map.insert(Key(2), Value("Two".to_string()));

    let result = my_map.swap_remove(&Key(1));
    assert_eq!(result, Some(Value("One".to_string())));
    assert_eq!(my_map.data.len(), 1);
}

#[test]
fn test_swap_remove_non_existing_key() {
    let mut my_map = MyMap::new();
    my_map.insert(Key(1), Value("One".to_string()));
    
    let result = my_map.swap_remove(&Key(2));
    assert_eq!(result, None);
    assert_eq!(my_map.data.len(), 1);
}

#[test]
fn test_swap_remove_last_element() {
    let mut my_map = MyMap::new();
    my_map.insert(Key(1), Value("One".to_string()));
    
    let result = my_map.swap_remove(&Key(1));
    assert_eq!(result, Some(Value("One".to_string())));
    assert_eq!(my_map.data.len(), 0);
}

#[test]
fn test_swap_remove_with_multiple_elements() {
    let mut my_map = MyMap::new();
    my_map.insert(Key(1), Value("One".to_string()));
    my_map.insert(Key(2), Value("Two".to_string()));
    my_map.insert(Key(3), Value("Three".to_string()));

    let result = my_map.swap_remove(&Key(2));
    assert_eq!(result, Some(Value("Two".to_string())));
    assert_eq!(my_map.data.len(), 2);
}

#[test]
fn test_swap_remove_no_effect_if_already_empty() {
    let mut my_map = MyMap::new();
    
    let result = my_map.swap_remove(&Key(1));
    assert_eq!(result, None);
    assert_eq!(my_map.data.len(), 0);
}

