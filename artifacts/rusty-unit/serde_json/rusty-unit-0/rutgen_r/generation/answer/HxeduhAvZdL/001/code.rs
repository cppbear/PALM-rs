// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use serde_json::{json, Map, Value};
    use serde_json::map::Entry;

    let mut map = Map::new();
    map.insert("key1".to_owned(), json!(10));

    match map.entry("key1") {
        Entry::Occupied(mut occupied) => {
            let old_value = occupied.insert(json!(20));
            assert_eq!(old_value, json!(10)); // old value should be 10
            assert_eq!(occupied.get(), &json!(20)); // current value should be 20
        }
        Entry::Vacant(_) => panic!("Entry should be occupied"),
    }
}

#[test]
fn test_insert_vacant_entry() {
    use serde_json::{json, Map, Value};
    use serde_json::map::Entry;

    let mut map = Map::new();

    match map.entry("key2") {
        Entry::Occupied(_) => panic!("Entry should be vacant"),
        Entry::Vacant(vacant) => {
            vacant.insert(json!(30));
            assert_eq!(map.get("key2"), Some(&json!(30))); // should retrieve the inserted value
        }
    }
}

#[test]
#[should_panic]
fn test_insert_on_nonexistent_entry() {
    use serde_json::{json, Map, Value};
    use serde_json::map::Entry;

    let mut map = Map::new();
    // Attempt to insert into an unoccupied entry without any value initialized.
    match map.entry("nonexistent_key") {
        Entry::Occupied(_) => panic!("Entry should not be occupied"),
        Entry::Vacant(vacant) => {
            let _ = vacant.insert(json!(40));
            assert!(map.contains_key("nonexistent_key")); // should contain the key after insertion
        }
    }
}

