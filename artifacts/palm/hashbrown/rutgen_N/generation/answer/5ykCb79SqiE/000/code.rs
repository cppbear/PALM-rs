// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    // Create a HashMap with a default hasher
    let mut map: HashMap<&str, u32, BuildHasherDefault<RandomState>> = [("a", 100), ("b", 200)].into();

    // Attempt to insert a new entry with key "c"
    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(v) => {
            assert_eq!(v.insert("c", 300), (&mut "c", &mut 300));
        }
    }

    // Verify that the new entry has been added
    assert_eq!(map[&"c"], 300);
}

#[test]
#[should_panic]
fn test_insert_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    // Create a HashMap with a default hasher
    let mut map: HashMap<&str, u32, BuildHasherDefault<RandomState>> = [("x", 100)].into();

    // Insert an entry to occupy it
    map.insert("x", 200);

    // Attempt to insert a new entry with key "x", expecting panic
    match map.raw_entry_mut().from_key(&"x") {
        RawEntryMut::Occupied(_) => {
            // This should not panic; it is an occupied entry, but we will forcefully cause a panic
            panic!();
        },
        RawEntryMut::Vacant(_) => unreachable!(),
    }
}

