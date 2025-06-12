// Answer 0

#[test]
fn test_raw_entry_mut_creation() {
    use crate::hash_map::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.extend([("key1", 1), ("key2", 2)]);

    let raw_entry = map.raw_entry_mut();
    assert!(raw_entry.map.is_some()); // Ensure the raw entry builder is created
}

#[test]
fn test_raw_entry_mut_insert_existing_key() {
    use crate::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.extend([("key1", 10), ("key2", 20)]);

    let mut raw_entry = map.raw_entry_mut();

    match raw_entry.from_key(&"key1") {
        RawEntryMut::Vacant(_) => unreachable!(),
        RawEntryMut::Occupied(mut view) => {
            assert_eq!(view.get(), &10);
            let v = view.get_mut();
            *v += 5; // Update value
            assert_eq!(view.insert(15), 10); // Insert new value with old key
        }
    }

    assert_eq!(map.get(&"key1"), Some(&15));
    assert_eq!(map.len(), 2); // Ensure the length hasn't changed
}

#[test]
fn test_raw_entry_mut_insert_new_key() {
    use crate::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.extend([("key1", 10), ("key2", 20)]);

    let mut raw_entry = map.raw_entry_mut();
    let new_key = "key3";

    match raw_entry.from_key(&new_key) {
        RawEntryMut::Occupied(_) => unreachable!(),
        RawEntryMut::Vacant(view) => {
            let (k, v) = view.insert(new_key, 30);
            assert_eq!((*k, *v), (new_key, 30)); // Check inserted key-value
        }
    }

    assert_eq!(map.get(&new_key), Some(&30));
    assert_eq!(map.len(), 3); // Ensure the length has increased
}

#[test]
fn test_raw_entry_mut_remove_entry() {
    use crate::hash_map::{HashMap, RawEntryMut};

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.extend([("key1", 10), ("key2", 20)]);

    let mut raw_entry = map.raw_entry_mut();

    match raw_entry.from_key(&"key2") {
        RawEntryMut::Vacant(_) => unreachable!(),
        RawEntryMut::Occupied(view) => {
            assert_eq!(view.remove_entry(), ("key2", 20)); // Ensure removal
        }
    }

    assert_eq!(map.get(&"key2"), None); // Ensure key2 is deleted
    assert_eq!(map.len(), 2); // Length should decrease
}

