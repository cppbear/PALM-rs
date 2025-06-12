// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher_fn(&"existing_key"), "existing_value".to_string(), hasher);
    
    let entry = table.entry(hasher_fn(&"existing_key"), |x| x == "existing_key", hasher_fn);
    let occupied_entry = entry.or_insert_with(|| "default_value".to_string());
}

#[test]
fn test_or_insert_with_occupied_entry_edge_case() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher_fn(&"key_0"), "value_0".to_string(), hasher);
    
    let entry = table.entry(hasher_fn(&"key_0"), |x| x == "key_0", hasher_fn);
    let occupied_entry = entry.or_insert_with(|| "should_not_insert".to_string());
}

#[test]
fn test_or_insert_with_multiple_occupied_entries() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    for i in 1..=10 {
        table.insert_unique(hasher_fn(&format!("key_{}", i)), format!("value_{}", i), hasher);
    }

    for i in 1..=10 {
        let entry = table.entry(hasher_fn(&format!("key_{}", i)), |x| x == &format!("key_{}", i), hasher_fn);
        let occupied_entry = entry.or_insert_with(|| format!("should_not_insert_{}", i));
    }
}

