// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::hash::{Hash, BuildHasherDefault};

    let mut map: HashMap<&str, u32, BuildHasherDefault<fnv::FnvHasher>> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(v) => {
            let result = v.insert("c", 300);
            assert_eq!(result, (&mut "c", &mut 300));
        },
    }

    assert_eq!(map[&"c"], 300);
}

#[test]
fn test_insert_existing_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::hash::{Hash, BuildHasherDefault};

    let mut map: HashMap<&str, u32, BuildHasherDefault<fnv::FnvHasher>> = [("a", 100), ("c", 300)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Occupied(_) => {},
        RawEntryMut::Vacant(_) => panic!(),
    }
}

#[test]
fn test_insert_multiple_entries() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::hash::{Hash, BuildHasherDefault};

    let mut map: HashMap<&str, u32, BuildHasherDefault<fnv::FnvHasher>> = [("d", 400), ("e", 500)].into();

    for i in 0..10 {
        let key = format!("key{}", i);
        let value = (i * 100) as u32;
        
        match map.raw_entry_mut().from_key(&key) {
            RawEntryMut::Occupied(_) => panic!(),
            RawEntryMut::Vacant(v) => {
                let result = v.insert(key.as_str(), value);
                assert_eq!(result, (key.as_mut_str(), &mut value));
            },
        }
    }

    for i in 0..10 {
        let key = format!("key{}", i);
        assert_eq!(map[key.as_str()], (i * 100) as u32);
    }
}

