// Answer 0

#[test]
fn test_get_key_value_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected to find an occupied entry."),
        RawEntryMut::Occupied(o) => {
            let (key, value) = o.get_key_value();
            assert_eq!(key, &"a");
            assert_eq!(value, &100);
        }
    }
}

#[test]
#[should_panic]
fn test_get_key_value_vacant() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Vacant(_) => {
            panic!("Expected to find an occupied entry for key 'c'.");
        },
        RawEntryMut::Occupied(_) => {}
    }
}

