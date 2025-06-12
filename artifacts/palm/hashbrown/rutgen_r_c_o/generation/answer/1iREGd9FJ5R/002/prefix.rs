// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("test_key", 42);

    let entry = map.raw_entry_mut().from_key("test_key").insert("test_key", 100);
}

#[test]
fn test_insert_occupied_entry_with_different_value() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("test_key", 42);

    let entry = map.raw_entry_mut().from_key("test_key").insert("test_key", 84);
}

#[test]
fn test_insert_multiple_occupied_entries() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 10);
    map.insert("key2", 20);
    
    let entry1 = map.raw_entry_mut().from_key("key1").insert("key1", 15);
    let entry2 = map.raw_entry_mut().from_key("key2").insert("key2", 25);
}

