// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32, RandomState> = HashMap::new();
    map.insert("key1", 5);
    
    let entry = map.entry("key1");
    entry.or_insert(10);
}

#[test]
fn test_or_insert_with_different_value_when_occupied() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32, RandomState> = HashMap::new();
    map.insert("key2", 7);
    
    let entry = map.entry("key2");
    *entry.or_insert(15) *= 2;
}

#[test]
fn test_or_insert_with_new_key() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32, RandomState> = HashMap::new();
    
    let entry = map.entry("key3");
    entry.or_insert(20);
    assert_eq!(map["key3"], 20);
}

#[test]
fn test_or_insert_with_existing_key_default_not_existing() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32, RandomState> = HashMap::new();
    map.insert("key4", 3);
    
    let entry = map.entry("key4");
    *entry.or_insert(50) += 10;
    assert_eq!(map["key4"], 13);
}

#[test]
fn test_or_insert_with_multiple_insertions() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32, RandomState> = HashMap::new();
    map.insert("key5", 1);

    let entry = map.entry("key5");
    entry.or_insert(2);
    *entry.or_insert(3) += 5; 
    assert_eq!(map["key5"], 6);
}

#[test]
fn test_or_insert_empty_map() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32, RandomState> = HashMap::new();
    
    let entry = map.entry("key6");
    entry.or_insert(25);
    assert_eq!(map["key6"], 25);
}

