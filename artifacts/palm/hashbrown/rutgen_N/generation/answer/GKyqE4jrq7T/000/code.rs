// Answer 0

#[test]
fn test_get_from_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => assert_eq!(o.get(), &100),
    }
}

#[test]
fn test_get_from_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(_) => assert!(true), // Just check it is vacant
    }
}

#[test]
#[should_panic]
fn test_get_from_vacant_entry_panics() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Occupied(o) => {
            let _ = o.get(); // Should panic if this is called on a vacant entry
        },
        RawEntryMut::Vacant(_) => panic!(),
    }
}

