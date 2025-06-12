// Answer 0

#[test]
fn test_key_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected entry to be occupied!"),
        RawEntryMut::Occupied(o) => {
            let key = o.key();
            assert_eq!(key, &"a");
        }
    }
}

#[test]
fn test_key_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Vacant(_) => {},  // Expecting this case to occur
        RawEntryMut::Occupied(_) => panic!("Expected entry to be vacant!"),
    }
}

