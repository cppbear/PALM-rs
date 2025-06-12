// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hash_fn = |val: &str| hasher.hash_one(val);

    table.insert_unique(hash_fn("poneyland"), "poneyland".to_string(), hasher);

    let entry = table.entry(hash_fn("poneyland"), |x| x == "poneyland", hasher);
    let occupied_entry = entry.or_insert_with(|| "default".to_string());

    assert_eq!(occupied_entry.get(), &"poneyland".to_string());
}

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hash_fn = |val: &str| hasher.hash_one(val);

    let entry = table.entry(hash_fn("poneyland"), |x| x == "poneyland", hasher);
    let occupied_entry = entry.or_insert_with(|| "poneyland".to_string());

    assert_eq!(occupied_entry.get(), &"poneyland".to_string());
    assert!(table.find(hash_fn("poneyland"), |x| x == "poneyland").is_some());
}

