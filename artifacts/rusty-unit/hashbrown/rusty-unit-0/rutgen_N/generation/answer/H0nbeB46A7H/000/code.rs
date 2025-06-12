// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    
    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected entry to be occupied"),
        RawEntryMut::Occupied(mut o) => {
            let old_value = o.insert(1000);
            assert_eq!(old_value, 100);
        }
    }

    assert_eq!(map[&"a"], 1000);
}

#[test]
fn test_insert_non_existent_entry() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Vacant(v) => {
            let old_value = v.insert(300);
            assert_eq!(old_value, None);
            assert_eq!(map[&"c"], 300);
        }
        RawEntryMut::Occupied(_) => panic!("Expected entry to be vacant"),
    }
}

#[test]
fn test_insert_same_value() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};
    
    let mut map: HashMap<&str, u32> = [("a", 100)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected entry to be occupied"),
        RawEntryMut::Occupied(mut o) => {
            let old_value = o.insert(100);
            assert_eq!(old_value, 100);
        }
    }

    assert_eq!(map[&"a"], 100);
}

