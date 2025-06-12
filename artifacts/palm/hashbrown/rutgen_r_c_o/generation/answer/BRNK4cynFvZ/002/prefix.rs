// Answer 0

#[test]
fn test_or_insert_with_key_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("existing_key", 42);
    
    let entry = map.entry("existing_key");
    entry.or_insert_with_key(|key| key.len());
}

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    
    let entry = map.entry("new_key");
    entry.or_insert_with_key(|key| key.len());
    let inserted_value = map["new_key"];
    
    assert_eq!(inserted_value, 7);
}

#[test]
fn test_or_insert_with_key_vacant_entry_multiple() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    
    let entry_one = map.entry("first_key");
    entry_one.or_insert_with_key(|key| key.chars().count());

    let entry_two = map.entry("second_key");
    entry_two.or_insert_with_key(|key| key.chars().count());

    assert_eq!(map["first_key"], 9);
    assert_eq!(map["second_key"], 10);
}

#[test]
fn test_or_insert_with_key_overwrite() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("key_to_overwrite", 100);
    
    let entry = map.entry("key_to_overwrite");
    *entry.or_insert_with_key(|key| key.len()) *= 2;

    assert_eq!(map["key_to_overwrite"], 200);
}

