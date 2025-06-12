// Answer 0

#[test]
fn test_or_insert_with_nonexistent_key() {
    use hashbrown::hash_map::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    
    map.entry_ref("poneyland").or_insert(3);
    assert_eq!(map["poneyland"], 3);
}

#[test]
fn test_or_insert_with_existing_key() {
    use hashbrown::hash_map::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_string(), 3);
    
    *map.entry_ref("poneyland").or_insert(10) *= 2;
    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_with_default_value() {
    use hashbrown::hash_map::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    
    assert_eq!(map.entry_ref("unicorn").or_insert(5), &mut 5);
    assert_eq!(map["unicorn"], 5);
}

#[test]
fn test_or_insert_empty_map() {
    use hashbrown::hash_map::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    
    *map.entry_ref(&1).or_insert(10) += 5;
    assert_eq!(map[&1], 15);
}

#[test]
fn test_or_insert_with_different_key_type() {
    use hashbrown::hash_map::HashMap;

    let mut map: HashMap<String, i32> = HashMap::new();
    
    map.entry_ref("key").or_insert(42);
    assert_eq!(map["key"], 42);
}

