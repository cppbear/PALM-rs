// Answer 0

#[test]
fn test_remove_from_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let removed_value = o.remove();
            assert_eq!(removed_value, 100);
        }
    }
    assert_eq!(map.get(&"a"), None);
}

#[test]
#[should_panic]
fn test_remove_from_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100)].into();

    match map.raw_entry_mut().from_key(&"b") {
        RawEntryMut::Vacant(_) => {
            // Trying to remove from a vacant entry should panic
            RawEntryMut::Vacant(()).remove();
        }
        RawEntryMut::Occupied(_) => panic!(),
    }
}

