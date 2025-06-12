// Answer 0

#[test]
fn test_or_insert_with_nonexistent_key() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    let entry = map.entry("poneyland");
    entry.or_insert(3);
    
    assert_eq!(map["poneyland"], 3);
}

#[test]
fn test_or_insert_with_existing_key() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 3);
    let entry = map.entry("poneyland");
    *entry.or_insert(10) *= 2;

    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_multiple_insertions() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;

    let mut map: HashMap<&str, u32> = HashMap::new();
    let entry1 = map.entry("poneyland");
    entry1.or_insert(5);
    let entry2 = map.entry("poneyland");
    *entry2.or_insert(10) += 5;

    assert_eq!(map["poneyland"], 10);
}

