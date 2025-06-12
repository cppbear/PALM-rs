// Answer 0

#[test]
fn test_or_insert_with_empty_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    // Create a Vacant entry for "poneyland"
    let vacant_entry = table.entry(hasher_fn("poneyland"), |x| x == "poneyland", |val| hasher_fn(val));

    // Invoke or_insert_with function
    let occupied_entry = vacant_entry.or_insert_with(|| "poneyland".to_string());

    // Check that the entry is now occupied
    assert!(table.find(hasher_fn(&"poneyland"), |x| x == "poneyland").is_some());
    assert_eq!(occupied_entry.get(), &"poneyland".to_string());
}

#[test]
fn test_or_insert_with_return_value() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    // Create a Vacant entry for "test_value"
    let vacant_entry = table.entry(hasher_fn("test_value"), |x| x == "test_value", |val| hasher_fn(val));

    // Insert a value using or_insert_with
    let returned_value = vacant_entry.or_insert_with(|| "test_value".to_string());

    // Verify that the returned value points to the correct occupied entry
    assert_eq!(returned_value.get(), &"test_value".to_string());
}

#[should_panic]
#[test]
fn test_or_insert_with_panic() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    // Create a Vacant entry for "panic_test"
    let vacant_entry = table.entry(hasher_fn("panic_test"), |x| x == "panic_test", |val| hasher_fn(val));

    // This will not panic since it's a default case, but simulate the panic by calling with faulty logic.
    vacant_entry.or_insert_with(|| panic!("This is a test panic!"));
}

