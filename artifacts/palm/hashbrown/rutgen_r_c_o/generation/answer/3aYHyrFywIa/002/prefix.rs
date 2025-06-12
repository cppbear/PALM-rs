// Answer 0

#[test]
fn test_or_default_occupied_entry() {
    let mut map: HashMap<u32, Option<u32>> = HashMap::new();
    map.insert(1, Some(10));
    let entry = map.entry(1);
    let value = entry.or_default();
}

#[test]
fn test_or_default_vacant_entry() {
    let mut map: HashMap<u32, Option<u32>> = HashMap::new();
    let entry = map.entry(2);
    let value = entry.or_default();
}

#[test]
fn test_or_default_multiple_entries() {
    let mut map: HashMap<u32, Option<u32>> = HashMap::new();
    map.insert(3, Some(20));
    map.insert(4, Some(30));
    let entry = map.entry(3);
    let value = entry.or_default();
    let new_entry = map.entry(5);
    let new_value = new_entry.or_default();
}

#[test]
fn test_or_default_with_default() {
    let mut map: HashMap<u32, Option<u32>> = HashMap::new();
    map.insert(6, None);
    let entry = map.entry(6);
    let value = entry.or_default();
}

#[test]
fn test_or_default_replace_existing() {
    let mut map: HashMap<u32, Option<u32>> = HashMap::new();
    map.insert(7, Some(15));
    let entry = map.entry(7);
    let value_updated = entry.or_default();
    let new_value = entry.or_default();
}

