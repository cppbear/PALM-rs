// Answer 0

#[test]
fn test_remove_from_non_empty_map() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_owned(), json!(42));
    match map.entry("key1") {
        Entry::Occupied(occupied) => {
            occupied.remove();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
#[should_panic]
fn test_remove_from_empty_map() {
    let map = serde_json::Map::new();
    match map.entry("invalid_key") {
        Entry::Occupied(occupied) => {
            occupied.remove();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_preserved_order_map_swap() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_owned(), json!(42));
    map.insert("key2".to_owned(), json!(true));
    match map.entry("key1") {
        Entry::Occupied(occupied) => {
            occupied.remove();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
fn test_remove_preserved_order_map_shift() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_owned(), json!(42));
    map.insert("key2".to_owned(), json!(true));
    map.insert("key3".to_owned(), json!(3.14));
    match map.entry("key2") {
        Entry::Occupied(occupied) => {
            occupied.remove();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

#[test]
#[should_panic]
fn test_remove_from_vacant_entry() {
    let mut map = serde_json::Map::new();
    match map.entry("invalid_key") {
        Entry::Occupied(occupied) => {
            occupied.remove();
        }
        Entry::Vacant(_) => unimplemented!(),
    }
}

