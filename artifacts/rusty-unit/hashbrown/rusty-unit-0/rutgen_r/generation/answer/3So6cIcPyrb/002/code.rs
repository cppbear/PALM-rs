// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::{BuildHasher, Hasher};
    
    struct MyStruct {
        value: String,
    }

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let mut hasher_fn = |val: &str| {
        let mut state = hasher.build_hasher();
        state.write(val.as_bytes());
        state.finish()
    };

    // Insert a value to create an occupied entry
    table.insert(hasher_fn("poneyland").to_string(), "poneyland".to_string());
    let occupied_entry = table.entry(hasher_fn("poneyland").to_string(), |x| x == "poneyland", |_| hasher_fn("poneyland"));

    // Ensure that we can obtain the occupied entry and value as expected
    let entry = occupied_entry.or_insert_with(|| "default_value".to_string());
    assert_eq!(entry, &"poneyland".to_string());
}

#[test]
#[should_panic]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::{BuildHasher, Hasher};

    struct MyStruct {
        value: String,
    }

    let mut table: HashTable<String> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &str| {
        let mut state = hasher.build_hasher();
        state.write(val.as_bytes());
        state.finish()
    };

    // Directly calling entry on a non-existent key, which leads to panic on a vacant entry
    let occupied_entry = table.entry(hasher_fn("nonexistent").to_string(), |x| x == "nonexistent", |_| hasher_fn("poneyland"));

    // This line should panic if it is truly a vacant entry
    let _ = occupied_entry.or_insert_with(|| "default_value".to_string());
}

