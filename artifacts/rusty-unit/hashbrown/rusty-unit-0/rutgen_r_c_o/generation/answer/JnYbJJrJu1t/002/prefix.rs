// Answer 0

#[test]
fn test_and_replace_entry_with_valid_key() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("existing_key", 100);

    let entry = map
        .raw_entry_mut()
        .from_key("existing_key")
        .and_replace_entry_with(|k, v| {
            assert_eq!(k, &"existing_key");
            assert_eq!(v, 100);
            Some(v + 1)
        });

    match entry {
        RawEntryMut::Occupied(e) => {
            assert_eq!(e.key(), &"existing_key");
            assert_eq!(e.get(), &101);
        },
        RawEntryMut::Vacant(_) => panic!(),
    }
    assert_eq!(map["existing_key"], 101);
}

#[test]
#[should_panic]
fn test_and_replace_entry_with_panic_function() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("existing_key", 100);

    let entry = map
        .raw_entry_mut()
        .from_key("existing_key")
        .and_replace_entry_with(|_k, _v| panic!("panic"));
    
    match entry {
        RawEntryMut::Occupied(_) => {},
        RawEntryMut::Vacant(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_return_none() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("existing_key", 100);

    let entry = map
        .raw_entry_mut()
        .from_key("existing_key")
        .and_replace_entry_with(|_k, _v| None);
    
    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
    assert!(!map.contains_key("existing_key"));
}

#[test]
fn test_and_replace_entry_with_invalid_key() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("existing_key", 100);

    let entry = map
        .raw_entry_mut()
        .from_key("non_existing_key")
        .and_replace_entry_with(|_k, _v| Some(200));
    
    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

