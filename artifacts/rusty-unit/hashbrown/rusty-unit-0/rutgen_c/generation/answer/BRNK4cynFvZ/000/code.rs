// Answer 0

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use crate::hash_map::HashMap;
    
    let mut map: HashMap<&str, usize> = HashMap::new();
    let entry = map.entry("poneyland");
    let value = entry.or_insert_with_key(|key| key.chars().count());
    
    assert_eq!(*value, 9);
    assert_eq!(map["poneyland"], 9);
}

#[test]
fn test_or_insert_with_key_occupied_entry() {
    use crate::hash_map::HashMap;
    
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("poneyland", 9);
    let entry = map.entry("poneyland");
    let value = entry.or_insert_with_key(|key| key.chars().count() * 10);
    
    assert_eq!(*value, 9);
    assert_eq!(map["poneyland"], 9);
    
    *value *= 2;
    assert_eq!(map["poneyland"], 18);
}

