// Answer 0

#[test]
fn test_insert_from_empty_map() {
    let mut map = HeaderMap::with_capacity(10);
    let key = "host".parse().unwrap();
    let value = "earth".parse().unwrap();

    if let Entry::Occupied(mut entry) = map.entry(key) {
        let prev = entry.insert(value);
        assert_eq!(prev, "earth"); // Should return the previously occupied value, which is none
    }
}

#[test]
fn test_insert_overwriting_existing_entry() {
    let mut map = HeaderMap::with_capacity(10);
    let key = "host".parse().unwrap();
    let initial_value = "hello.world".parse().unwrap();
    let new_value = "earth".parse().unwrap();

    map.insert(key.clone(), initial_value);
    if let Entry::Occupied(mut entry) = map.entry(key) {
        let prev = entry.insert(new_value);
        assert_eq!(prev, "hello.world"); // Should return the previously occupied value
    }
    assert_eq!(map.get(key).unwrap().to_string(), "earth"); // The new value should be inserted
}

#[test]
fn test_insert_multiple_values() {
    let mut map = HeaderMap::with_capacity(10);
    let key = "host".parse().unwrap();
    let value1 = "hello.world".parse().unwrap();
    let value2 = "earth".parse().unwrap();

    map.insert(key.clone(), value1);
    if let Entry::Occupied(mut entry) = map.entry(key) {
        entry.append(value2); // Appending a second value should not panic
        let prev = entry.insert("moon".parse().unwrap());
        assert_eq!(prev.clone(), "hello.world"); // Should return value before insertion
    }
    assert_eq!(map.get(key).unwrap().to_string(), "moon"); // Check for final inserted value
}

#[test]
#[should_panic]
fn test_insert_panic_on_non_existent_entry() {
    let mut map = HeaderMap::with_capacity(10);
    let key = "host".parse().unwrap();
    
    // Attempt to insert into a non-existent entry
    let value = "earth".parse().unwrap();
    let _ = map.entry(key).unwrap(); // This line should panic
}

#[test]
fn test_insert_boundary_condition() {
    let max_size = MAX_SIZE as usize;
    let mut map = HeaderMap::with_capacity(max_size);
    let key = "host".parse().unwrap();
    
    for i in 0..max_size {
        let value = format!("value_{}", i).parse().unwrap();
        map.insert(key.clone(), value);
    }

    // After inserting max size check the length
    assert_eq!(map.len(), max_size);
}

