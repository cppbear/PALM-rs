// Answer 0

#[test]
fn test_remove_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("x", 300), ("y", 400)].into();

    match map.raw_entry_mut().from_key(&"x") {
        RawEntryMut::Vacant(_) => panic!("Expected entry to be occupied"),
        RawEntryMut::Occupied(o) => {
            assert_eq!(o.remove(), 300);
        }
    }
    assert_eq!(map.get(&"x"), None);
}

#[test]
#[should_panic(expected = "Expected entry to be occupied")]
fn test_remove_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 500)].into();

    match map.raw_entry_mut().from_key(&"b") {
        RawEntryMut::Vacant(_) => panic!("Expected entry to be occupied"),
        RawEntryMut::Occupied(o) => {
            o.remove(); // This should not be reached
        }
    }
}

#[test]
fn test_remove_multiple_entries() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("key1", 1), ("key2", 2), ("key3", 3)].into();

    for key in ["key1", "key2", "key3"].iter() {
        match map.raw_entry_mut().from_key(key) {
            RawEntryMut::Vacant(_) => panic!("Expected entry to be occupied for key {}", key),
            RawEntryMut::Occupied(o) => {
                assert_eq!(o.remove(), match *key {
                    "key1" => 1,
                    "key2" => 2,
                    "key3" => 3,
                    _ => unreachable!(),
                });
            }
        }
        assert_eq!(map.get(key), None);
    }
}

