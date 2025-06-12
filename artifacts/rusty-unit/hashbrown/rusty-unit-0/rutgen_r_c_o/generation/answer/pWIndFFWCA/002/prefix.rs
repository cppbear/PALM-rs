// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    let mut map: hashbrown::HashMap<String, u32> = hashbrown::HashMap::new();
    map.insert("key1".to_owned(), 5);

    let entry_ref = map.entry_ref("key1");
    entry_ref.or_insert(10);
}

#[test]
fn test_or_insert_with_vacant_entry_and_insert() {
    let mut map: hashbrown::HashMap<String, u32> = hashbrown::HashMap::new();
    let entry_ref = map.entry_ref("key2");
    entry_ref.or_insert(15);
}

#[test]
fn test_or_insert_with_vacant_entry_and_existing_key() {
    let mut map: hashbrown::HashMap<String, u32> = hashbrown::HashMap::new();
    map.insert("key3".to_owned(), 20);

    let entry_ref = map.entry_ref("key3");
    entry_ref.or_insert(25);
}

#[test]
fn test_or_insert_with_vacant_entry_that_was_previous_occupied() {
    let mut map: hashbrown::HashMap<String, u32> = hashbrown::HashMap::new();
    map.insert("key4".to_owned(), 30);

    {
        let entry_ref = map.entry_ref("key4");
        entry_ref.or_insert(35);
    }

    {
        let entry_ref = map.entry_ref("key4");
        entry_ref.or_insert(40);
    }
}

#[test]
fn test_or_insert_without_existing_key() {
    let mut map: hashbrown::HashMap<String, u32> = hashbrown::HashMap::new();
    let entry_ref = map.entry_ref("key5");
    entry_ref.or_insert(50);
}

