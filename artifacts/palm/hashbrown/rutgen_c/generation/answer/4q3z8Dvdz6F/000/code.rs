// Answer 0

#[test]
fn test_and_modify_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    match map.raw_entry_mut().from_key("poneyland") {
        RawEntryMut::Occupied(entry) => {
            entry.and_modify(|_k, v| { *v += 1; });
            assert_eq!(entry.get(), &42);
        },
        RawEntryMut::Vacant(_) => panic!("Expected an occupied entry"),
    }

    assert_eq!(map["poneyland"], 43);
}

#[test]
fn test_and_modify_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();

    match map.raw_entry_mut().from_key("vacantland") {
        RawEntryMut::Occupied(_) => panic!("Expected a vacant entry"),
        RawEntryMut::Vacant(entry) => {
            entry.and_modify(|_k, v| { *v += 1; });
            entry.or_insert("vacantland", 0);
        },
    }

    assert_eq!(map["vacantland"], 0);
}

#[test]
fn test_and_modify_multiple_times() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 1);

    for _ in 0..3 {
        match map.raw_entry_mut().from_key("poneyland") {
            RawEntryMut::Occupied(entry) => {
                entry.and_modify(|_k, v| { *v += 1; });
            },
            RawEntryMut::Vacant(_) => panic!("Expected an occupied entry"),
        }
    }

    assert_eq!(map["poneyland"], 4);
}

