// Answer 0

#[test]
fn test_replace_entry_with_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("a", 100);
    map.insert("b", 200);

    let raw_entry = match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 100);
            Some(v + 900) // This will make the value 1000
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => {
            let new_raw_entry = o.replace_entry_with(|k, v| {
                assert_eq!(k, &"a");
                assert_eq!(v, 1000);
                None // This will remove the entry
            });
            match new_raw_entry {
                RawEntryMut::Vacant(_) => {},
                RawEntryMut::Occupied(_) => panic!(),
            }
        },
    };

    assert_eq!(map.get(&"a"), None);
}

