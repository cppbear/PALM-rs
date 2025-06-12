// Answer 0

#[test]
fn test_get_mut_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            *o.get_mut() += 900;
        }
    }

    assert_eq!(map[&"a"], 1000);
}

#[test]
#[should_panic]
fn test_get_mut_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_get_mut_boundary_condition() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(1, 1);

    match map.raw_entry_mut().from_key(&1) {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(mut o) => {
            *o.get_mut() *= 10; // Changing value to 10
        }
    }

    assert_eq!(map[&1], 10);
}

