// Answer 0

#[test]
fn test_swap_remove_existing_key() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let removed_value = map.swap_remove(&2);
    assert_eq!(removed_value, Some("two"));
    assert_eq!(map.len(), 2);
    assert!(!map.contains_key(&2));
}

#[test]
fn test_swap_remove_non_existing_key() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    
    let removed_value = map.swap_remove(&3);
    assert_eq!(removed_value, None);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_swap_remove_last_element() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");

    let removed_value = map.swap_remove(&1);
    assert_eq!(removed_value, Some("one"));
    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&2));
}

#[test]
fn test_swap_remove_empty_map() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::new();
    
    let removed_value = map.swap_remove(&1);
    assert_eq!(removed_value, None);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_swap_remove_with_different_key_type() {
    use indexmap::IndexMap;
    use std::collections::hash_map::RandomState;

    struct CustomKey(i32);

    impl PartialEq for CustomKey {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for CustomKey {}

    use std::hash::{Hash, Hasher};
    
    impl Hash for CustomKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    let mut map: IndexMap<CustomKey, &str, RandomState> = IndexMap::new();
    map.insert(CustomKey(1), "one");
    map.insert(CustomKey(2), "two");

    let removed_value = map.swap_remove(&CustomKey(1));
    assert_eq!(removed_value, Some("one"));
    assert_eq!(map.len(), 1);
    assert!(!map.contains_key(&CustomKey(1)));
}

