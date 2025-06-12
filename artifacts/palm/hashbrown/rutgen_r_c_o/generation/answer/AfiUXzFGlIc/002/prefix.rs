// Answer 0

#[test]
fn test_or_insert_with_occupied_entry_key1() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 1);
    let entry = map.entry("key1");
    entry.or_insert_with(|| 10);
}

#[test]
fn test_or_insert_with_occupied_entry_key2() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key2", 10);
    let entry = map.entry("key2");
    entry.or_insert_with(|| 100);
}

#[test]
fn test_or_insert_with_occupied_entry_key1_large_value() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 100);
    let entry = map.entry("key1");
    entry.or_insert_with(|| 1000);
}

#[test]
fn test_or_insert_with_occupied_entry_key2_zero_value() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key2", 0);
    let entry = map.entry("key2");
    entry.or_insert_with(|| 1);
}

#[test]
fn test_or_insert_with_multiple_updates_key1() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 10);
    let entry = map.entry("key1");
    entry.or_insert_with(|| 100);
    *entry.or_insert_with(|| 200) *= 2;
}

