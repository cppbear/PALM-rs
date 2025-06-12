// Answer 0

#[test]
fn test_replace_entry_with_updates_value() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100)].into();

    let raw_entry = match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 100);
            Some(v + 900)
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 1000);
            None
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    };

    assert_eq!(map.get(&"a"), None);
}

#[test]
fn test_replace_entry_with_no_update() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("b", 200)].into();

    let raw_entry = match map.raw_entry_mut().from_key(&"b") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"b");
            assert_eq!(v, 200);
            None
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    };

    assert_eq!(map.get(&"b"), None);
}

#[test]
#[should_panic]
fn test_replace_entry_with_panic() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("c", 300)].into();

    // Trying to replace a non-existent entry
    let raw_entry = match map.raw_entry_mut().from_key(&"d") {
        RawEntryMut::Vacant(_) => panic!("Entry does not exist"),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|_, _| Some(400)),
    };
}

