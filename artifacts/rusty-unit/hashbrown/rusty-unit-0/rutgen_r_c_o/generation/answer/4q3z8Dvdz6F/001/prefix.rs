// Answer 0

#[test]
fn test_and_modify_vacant_entry() {
    use hashbrown::{HashMap, raw::RawEntryMut};
    
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("test", 1);

    let entry = map.raw_entry_mut().from_key("non_existent_key");
    entry.and_modify(|_k, v| { *v += 1; });
}

#[test]
fn test_and_modify_vacant_entry_with_empty_map() {
    use hashbrown::{HashMap, raw::RawEntryMut};

    let mut map: HashMap<&str, i32> = HashMap::new();

    let entry = map.raw_entry_mut().from_key("another_key");
    entry.and_modify(|_k, v| { *v += 10; });
}

#[test]
fn test_and_modify_vacant_entry_non_empty() {
    use hashbrown::{HashMap, raw::RawEntryMut};

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("key_one", 2);

    let entry = map.raw_entry_mut().from_key("key_two");
    entry.and_modify(|_k, v| { *v += 5; });
}

