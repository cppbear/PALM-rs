// Answer 0

#[test]
fn test_replace_entry_with_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    let raw_entry = match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected the entry to be occupied."),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 100);
            Some(v + 900) // Expecting a new value of 1000
        }),
    };

    let raw_entry = match raw_entry {
        RawEntryMut::Vacant(_) => panic!("Expected the entry to remain occupied."),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 1000);
            None // This will set the entry to vacant
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => { }, // This is expected after removal
        RawEntryMut::Occupied(_) => panic!("Expected the entry to be vacant."),
    };

    assert_eq!(map.get(&"a"), None); // Entry should be removed and return None
}

#[test]
fn test_replace_entry_with_multiple_updates() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 300), ("b", 400)].into();

    let raw_entry = match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected the entry to be occupied."),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 300);
            Some(v + 700) // Expecting a new value of 1000
        }),
    };

    let raw_entry = match raw_entry {
        RawEntryMut::Vacant(_) => panic!("Expected the entry to remain occupied."),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 1000);
            Some(v + 1000) // Expecting a new value of 2000
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => panic!("Expected the entry to remain occupied again."),
        RawEntryMut::Occupied(o) => {
            // Final update to remove
            let _ = o.replace_entry_with(|k, v| {
                assert_eq!(k, &"a");
                assert_eq!(v, 2000);
                None // Remove the entry
            });
        },
    };

    assert_eq!(map.get(&"a"), None); // Entry should be removed and return None
}

