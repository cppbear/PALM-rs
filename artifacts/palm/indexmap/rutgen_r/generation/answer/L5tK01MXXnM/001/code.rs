// Answer 0

#[test]
fn test_from_key_existing_entry() {
    use indexmap::IndexMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut map: IndexMap<String, i32, std::hash::BuildHasherDefault<DefaultHasher>> = IndexMap::new();
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);

    let entry = map.from_key(&"key1".to_string());
    assert_eq!(entry, Some((&"key1".to_string(), &10)));
}

#[test]
fn test_from_key_non_existing_entry() {
    use indexmap::IndexMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut map: IndexMap<String, i32, std::hash::BuildHasherDefault<DefaultHasher>> = IndexMap::new();
    map.insert("key1".to_string(), 10);
    
    let entry = map.from_key(&"key2".to_string());
    assert_eq!(entry, None);
}

#[test]
#[should_panic]
fn test_from_key_panic_with_invalid_key_type() {
    use indexmap::IndexMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut map: IndexMap<String, i32, std::hash::BuildHasherDefault<DefaultHasher>> = IndexMap::new();
    map.insert("key1".to_string(), 10);

    // Attempting to pass a key of a different type that doesn't satisfy the parameter bounds
    let _: Option<(&String, &i32)> = map.from_key(&100);
}

