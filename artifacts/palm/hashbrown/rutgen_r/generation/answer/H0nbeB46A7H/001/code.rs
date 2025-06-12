// Answer 0

#[test]
fn test_insert_existing_value() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected occupied entry for key 'a'"),
        RawEntryMut::Occupied(mut o) => {
            assert_eq!(o.insert(500), 100);
            assert_eq!(map[&"a"], 500);
        }
    }
}

#[test]
fn test_insert_another_existing_value() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("x", 300), ("y", 400)].into();

    match map.raw_entry_mut().from_key(&"y") {
        RawEntryMut::Vacant(_) => panic!("Expected occupied entry for key 'y'"),
        RawEntryMut::Occupied(mut o) => {
            assert_eq!(o.insert(800), 400);
            assert_eq!(map[&"y"], 800);
        }
    }
}

#[test]
fn test_insert_key_not_found() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("m", 1000), ("n", 2000)].into();

    match map.raw_entry_mut().from_key(&"o") {
        RawEntryMut::Vacant(_) => {
            // When the entry is vacant, we don't want to panic, and we can't call insert.
            // So we will panic explicitly if we reach this point.
            panic!("Expected occupied entry for key 'o'");
        },
        RawEntryMut::Occupied(_) => unreachable!(),
    }
}

#[test]
fn test_insert_same_value() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, u32> = [("k", 50)].into();

    match map.raw_entry_mut().from_key(&"k") {
        RawEntryMut::Vacant(_) => panic!("Expected occupied entry for key 'k'"),
        RawEntryMut::Occupied(mut o) => {
            assert_eq!(o.insert(50), 50);
            assert_eq!(map[&"k"], 50);
        }
    }
}

