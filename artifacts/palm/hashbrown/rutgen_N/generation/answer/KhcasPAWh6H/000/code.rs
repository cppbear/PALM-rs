// Answer 0

#[test]
fn test_hash_map_new_in_empty() {
    use hashbrown::HashMap;
    use bumpalo::Bump;

    let bump = Bump::new();
    let map: HashMap<&str, i32> = HashMap::new_in(&bump);

    // The created HashMap holds no elements
    assert_eq!(map.len(), 0);
    
    // The created HashMap also doesn't allocate memory
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_hash_map_new_in_insert_element() {
    use hashbrown::HashMap;
    use bumpalo::Bump;

    let bump = Bump::new();
    let mut map: HashMap<&str, i32> = HashMap::new_in(&bump);

    // Now we insert an element into the created HashMap
    map.insert("One", 1);
    
    // We can see that the HashMap holds 1 element
    assert_eq!(map.len(), 1);
    
    // And it also allocates some capacity
    assert!(map.capacity() > 1);
}

