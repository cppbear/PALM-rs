// Answer 0

#[test]
fn test_raw_entry_get_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected entry to be occupied"),
        RawEntryMut::Occupied(o) => {
            let value: &u32 = o.get();
            assert_eq!(value, &100);
        }
    }
}

#[test]
#[should_panic(expected = "Expected entry to be occupied")]
fn test_raw_entry_get_vacant() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();

    match map.raw_entry_mut().from_key(&"non_existent_key") {
        RawEntryMut::Vacant(_) => {}, // This will not panic
        RawEntryMut::Occupied(_) => panic!("Expected entry to be vacant"),
    };
}

