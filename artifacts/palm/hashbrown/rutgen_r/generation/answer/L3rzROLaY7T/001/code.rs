// Answer 0

#[test]
fn test_remove_entry_occupied() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected entry for key 'a' to be occupied"),
        RawEntryMut::Occupied(o) => {
            assert_eq!(o.remove_entry(), ("a", 100));
        },
    }
    assert_eq!(map.get(&"a"), None);
}

#[test]
#[should_panic(expected = "Expected entry for key 'c' to be occupied")]
fn test_remove_entry_vacant() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Vacant(_) => panic!("Expected entry for key 'c' to be occupied"),
        RawEntryMut::Occupied(_) => {},
    }
}

