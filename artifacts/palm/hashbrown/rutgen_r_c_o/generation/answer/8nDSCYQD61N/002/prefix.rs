// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("first".to_string(), 10);
    map.insert("second".to_string(), 20);

    match map.entry_ref("first") {
        EntryRef::Occupied(entry) => {
            entry.and_modify(|val| *val += 5);
        }
        _ => panic!("Expected occupied entry"),
    }

    match map.entry_ref("second") {
        EntryRef::Occupied(entry) => {
            entry.and_modify(|val| *val += 10);
        }
        _ => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_and_modify_multiple_times() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("apple".to_string(), 1);

    for _ in 0..5 {
        match map.entry_ref("apple") {
            EntryRef::Occupied(entry) => {
                entry.and_modify(|val| *val *= 2);
            }
            _ => panic!("Expected occupied entry"),
        }
    }
}

#[test]
fn test_and_modify_edge_case() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("edge_case".to_string(), 100);

    match map.entry_ref("edge_case") {
        EntryRef::Occupied(entry) => {
            entry.and_modify(|val| *val = 0);
        }
        _ => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_and_modify_with_different_keys() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);

    match map.entry_ref("key1") {
        EntryRef::Occupied(entry) => {
            entry.and_modify(|val| *val += 10);
        }
        _ => panic!("Expected occupied entry"),
    }

    match map.entry_ref("key2") {
        EntryRef::Occupied(entry) => {
            entry.and_modify(|val| *val -= 1);
        }
        _ => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_and_modify_complex_logic() {
    use hashbrown::hash_map::{EntryRef, HashMap};

    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("complex".to_string(), 100);

    match map.entry_ref("complex") {
        EntryRef::Occupied(entry) => {
            entry.and_modify(|val| {
                if *val < 100 {
                    *val += 50;
                } else {
                    *val -= 50;
                }
            });
        }
        _ => panic!("Expected occupied entry"),
    }
}

