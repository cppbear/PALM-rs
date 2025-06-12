// Answer 0

#[test]
fn test_get_mut_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected entry to be occupied"),
        RawEntryMut::Occupied(mut o) => {
            let value = o.get_mut();
            *value += 900;
        }
    }

    assert_eq!(map[&"a"], 1000);
}

#[test]
#[should_panic(expected = "Expected entry to be occupied")]
fn test_get_mut_vacant() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Vacant(_) => {} // This is expected
        RawEntryMut::Occupied(_) => panic!("Expected entry to be vacant"),
    }
}

#[test]
fn test_get_mut_multiple_entries() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 1), ("b", 2), ("c", 3)].into();

    match map.raw_entry_mut().from_key(&"b") {
        RawEntryMut::Vacant(_) => panic!("Expected entry to be occupied"),
        RawEntryMut::Occupied(mut o) => {
            let value = o.get_mut();
            *value += 5;
        }
    }

    assert_eq!(map[&"b"], 7);
}

