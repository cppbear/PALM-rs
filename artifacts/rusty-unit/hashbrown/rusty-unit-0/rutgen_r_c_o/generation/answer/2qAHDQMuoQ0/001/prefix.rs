// Answer 0

#[test]
fn test_into_mut_with_existing_entry() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 10);
    let value: &mut i32;
    match map.entry(1) {
        Occupied(entry) => value = entry.into_mut(),
        Vacant(_) => panic!(),
    }
    *value += 5;
}

#[test]
fn test_into_mut_with_multiple_entries() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("apple".to_string(), 5);
    map.insert("banana".to_string(), 15);
    
    let value: &mut i32;
    match map.entry("banana".to_string()) {
        Occupied(entry) => value = entry.into_mut(),
        Vacant(_) => panic!(),
    }
    *value += 10;
}

#[test]
fn test_into_mut_with_large_values() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 1..=1000 {
        map.insert(i, i * 2);
    }
    let value: &mut i32;
    match map.entry(1000) {
        Occupied(entry) => value = entry.into_mut(),
        Vacant(_) => panic!(),
    }
    *value += 20;
}

#[test]
#[should_panic]
fn test_into_mut_with_non_existing_entry() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let value: &mut i32;
    match map.entry(999) {
        Occupied(_) => panic!(),
        Vacant(_) => {
            // Attempting to call into_mut on a non-existing entry should panic
            panic!("Expected panic, entry is vacant")
        }
    }
}

#[test]
fn test_into_mut_with_string_keys() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("key1", 100);
    
    let value: &mut i32;
    match map.entry("key1") {
        Occupied(entry) => value = entry.into_mut(),
        Vacant(_) => panic!(),
    }
    *value *= 2;
}

