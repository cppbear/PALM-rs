// Answer 0

#[derive(Default)]
struct MyMap {
    map: std::collections::HashMap<String, i32>,
}

impl MyMap {
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            iter: self.map.iter_mut(),
        }
    }
}

struct IterMut<'a> {
    iter: std::collections::hash_map::IterMut<'a, String, i32>,
}

#[test]
fn test_iter_mut_empty_map() {
    let mut my_map = MyMap::default();
    let mut iter = my_map.iter_mut();
    assert!(iter.iter.next().is_none());
}

#[test]
fn test_iter_mut_single_entry() {
    let mut my_map = MyMap::default();
    my_map.map.insert("key1".to_string(), 1);
    let mut iter = my_map.iter_mut();
    if let Some((key, value)) = iter.iter.next() {
        assert_eq!(key, "key1");
        *value += 1; // Ensure we can mutate the value
    }
    assert_eq!(my_map.map["key1"], 2);
}

#[test]
fn test_iter_mut_multiple_entries() {
    let mut my_map = MyMap::default();
    my_map.map.insert("key1".to_string(), 1);
    my_map.map.insert("key2".to_string(), 2);
    let mut iter = my_map.iter_mut();
    
    let mut keys_collected = Vec::new();
    let mut values_sum = 0;

    for (key, value) in iter.iter {
        keys_collected.push(key.clone());
        values_sum += *value;
        *value += 1; // Mutate the values
    }
    
    assert_eq!(keys_collected.len(), 2);
    assert_eq!(values_sum, 3);
    assert_eq!(my_map.map["key1"], 2);
    assert_eq!(my_map.map["key2"], 3);
}

#[test]
fn test_iter_mut_with_no_panic() {
    let mut my_map = MyMap::default();
    my_map.map.insert("key1".to_string(), 1);
    my_map.map.insert("key2".to_string(), 2);
    let mut iter = my_map.iter_mut();

    while let Some((_, value)) = iter.iter.next() {
        assert!(*value > 0); // Validate that values are positive
    }
}

