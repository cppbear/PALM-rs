// Answer 0

#[test]
fn test_get_existing_key() {
    let mut map: HashMap<&str, u32> = [("key1", 10), ("key2", 20)].into();
    match map.raw_entry_mut().from_key(&"key1") {
        RawEntryMut::Occupied(mut entry) => {
            entry.get();
        },
        _ => panic!(),
    }
}

#[test]
fn test_get_another_existing_key() {
    let mut map: HashMap<&str, u32> = [("keyA", 15), ("keyB", 25)].into();
    match map.raw_entry_mut().from_key(&"keyB") {
        RawEntryMut::Occupied(mut entry) => {
            entry.get();
        },
        _ => panic!(),
    }
}

#[test]
fn test_get_key_not_found() {
    let mut map: HashMap<&str, u32> = [("test1", 30), ("test2", 40)].into();
    match map.raw_entry_mut().from_key(&"nonexistent_key") {
        RawEntryMut::Vacant(_) => {},
        _ => panic!(),
    }
}

