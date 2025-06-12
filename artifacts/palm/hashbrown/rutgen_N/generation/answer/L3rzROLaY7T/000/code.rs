// Answer 0

#[test]
fn test_remove_entry_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let entry = o.remove_entry();
            assert_eq!(entry, ("a", 100));
        },
    }
    assert_eq!(map.get(&"a"), None);
}

#[test]
fn test_remove_entry_non_existent() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(_) => {}
    }
    assert_eq!(map.get(&"c"), None);
}

