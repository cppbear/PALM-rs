// Answer 0

#[test]
fn test_replace_entry_with_vacant_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};

    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);

    let raw_entry = match map.raw_entry_mut().from_key(&2) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &2);
            assert_eq!(v, 20);
            None
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => { },
        RawEntryMut::Occupied(_) => panic!(),
    };

    assert_eq!(map.get(&2), None);
}

#[test]
fn test_replace_entry_with_nonexistent_key() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};

    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(4, 40);
    map.insert(5, 50);

    let raw_entry = match map.raw_entry_mut().from_key(&6) {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(o) => o.replace_entry_with(|_k, _v| {
            Some(100)
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => { },
        RawEntryMut::Occupied(_) => panic!(),
    };

    assert_eq!(map.get(&6), None);
}

#[test]
fn test_replace_entry_with_remove_existing_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};

    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(7, 70);
    map.insert(8, 80);
    
    let raw_entry = match map.raw_entry_mut().from_key(&7) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &7);
            assert_eq!(v, 70);
            None
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => { },
        RawEntryMut::Occupied(_) => panic!(),
    };

    assert_eq!(map.get(&7), None);
}

