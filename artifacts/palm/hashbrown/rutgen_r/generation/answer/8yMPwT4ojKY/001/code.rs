// Answer 0


#[test]
fn test_into_mut_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let value: &mut u32;

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected occupied entry but found vacant."),
        RawEntryMut::Occupied(o) => value = o.into_mut(),
    }

    *value += 900;

    assert_eq!(map[&"a"], 1000);
}

#[test]
#[should_panic(expected = "Expected occupied entry but found vacant.")]
fn test_into_mut_vacant_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new(); // Creates a new empty map
    let _value: &mut u32;

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Vacant(_) => {
            panic!("Expected occupied entry but found vacant.");
        },
        RawEntryMut::Occupied(o) => {
            _value = o.into_mut();
        },
    }
}


